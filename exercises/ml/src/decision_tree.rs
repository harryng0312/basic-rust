use crate::number_utils::calc_euclidean;
use core::f64;
use linfa::prelude::{Fit, Transformer};
use linfa_preprocessing::PreprocessingError;
use log::info;
use maplit::hashmap;
use ndarray::{Array1, Array2, Axis};
use plotters::prelude::LogScalable;
use rand;
use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Map;
use utils::log::configuration::{init_logger, load_config_file};

fn min_max_scale(inp_arr: &Array2<f64>) -> Result<Array2<f64>, PreprocessingError> {
    let mins = inp_arr.fold_axis(Axis(0), f64::MAX, |&a, &b| a.min(b));
    let maxs = inp_arr.fold_axis(Axis(0), f64::MIN, |&a, &b| a.max(b));
    let scaled_arr = inp_arr.mapv(|x| {
        let col_min = mins[0];
        let col_max = maxs[0];

        (x - col_min) / (col_max - col_min)
    });

    Ok(scaled_arr)
}

struct Node {
    feature_idx: Option<usize>,
    threshold: Option<f64>,
    value: Option<f64>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
fn read_from_csv(file_name: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    // let mut reader = csv::Reader::from_reader(file);
    let reader = BufReader::new(file);
    let mut rs: Vec<Vec<String>> = vec![];
    for line in reader.lines() {
        let record = line?;
        let parts = record
            .splitn(2, ',')
            .map(|x| x.trim())
            .map(|x| x.strip_prefix('"').unwrap_or(x))
            .map(|x| x.strip_suffix('"').unwrap_or(x))
            .map(|x| x.trim())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        rs.push(parts);
    }
    rs.remove(0);
    Ok(rs)
}

fn extract_keywords(data: &Vec<Vec<String>>, top_n: u32) -> Result<Vec<String>, Box<dyn Error>> {
    let mut word_freq: HashMap<String, u32> = hashmap! {};
    for row in data {
        let msg = &row[1]
            .to_lowercase()
            .replace(",", " ")
            .replace(".", " ")
            .replace(":", " ")
            .replace(";", " ")
            .replace("!", " ")
            .replace("(", "")
            .replace(")", "")
            .replace("[", "")
            .replace("]", "");
        let words = msg
            .trim()
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>();
        for word in words {
            let freq = word_freq.entry(word.to_string()).or_insert(0);
            *freq += 1;
        }
    }
    // let word_fred_ls = word_freq.into_iter()
    //     .collect::<Vec<(&str, u32)>>();z

    let mut word_fred_vec = word_freq.iter().collect::<Vec<(&String, &u32)>>();
    word_fred_vec.sort_by(|a, b| b.1.cmp(a.1));

    let sorted_words = word_fred_vec[..min(top_n, word_fred_vec.len() as u32) as usize]
        .iter()
        .map(|x| x.0.to_string())
        .collect::<Vec<String>>();
    Ok(sorted_words)
}

fn extract_features(msg: &String, keywords: &Vec<String>) -> Result<Vec<u32>, Box<dyn Error>> {
    let message = msg.to_lowercase();
    let mut features: Vec<u32> = vec![0];

    for keyword in keywords {
        let count = message.matches(keyword).count();
        features.push(count as u32);
    }

    Ok(features)
}
fn preprocess_data(
    data: &Vec<Vec<String>>,
    keywords: &Vec<String>,
) -> Result<(Vec<Vec<u32>>, Vec<u32>), Box<dyn Error>> {
    let mut X_data: Vec<Vec<u32>> = vec![];
    let mut y_data: Vec<u32> = vec![];
    for row in data {
        let features = extract_features(&row[1], keywords)?;
        let label = if "spam" == row[0] { 1 } else { 0 };
        X_data.push(features);
        y_data.push(label);
    }

    Ok((X_data, y_data))
}
fn calculate_entropy(labels: &Vec<u32>) -> f64 {
    let len_n = labels.len();
    if len_n == 0 {
        return 0.0;
    }
    let unq_labels: HashSet<u32> = labels.iter().map(|x| *x).collect::<HashSet<u32>>();
    let mut entropy = 0.0;
    for label in unq_labels {
        let label_count = labels.iter().filter(|&x| *x == label).count() as f64;
        let p = label_count / len_n as f64;
        entropy -= p * p.log2();
    }

    entropy
}

fn calculate_infomation_gain(
    m_X_train: &Vec<Vec<u32>>,
    y_train: &Vec<u32>,
    feature_idx: usize,
    threshold: f64,
) -> (f64, Vec<Vec<u32>>, Vec<u32>, Vec<Vec<u32>>, Vec<u32>) {
    let n = m_X_train.len();
    let mut inf_gain = 0.0;
    let mut left_data = vec![];
    let mut left_labels = vec![];
    let mut right_data = vec![];
    let mut right_labels = vec![];
    if n == 0 {
        return (inf_gain, left_data, left_labels, right_data, right_labels);
    }

    // calculate parent - current entropy
    let parent_entropy = calculate_entropy(y_train);

    // split left and right by entropy
    for (data_row, label_row) in m_X_train.iter().zip(y_train.iter()) {
        if data_row[feature_idx] as f64 <= threshold {
            left_data.push(data_row.to_owned());
            left_labels.push(label_row.to_owned());
        } else {
            right_data.push(data_row.to_owned());
            right_labels.push(label_row.to_owned());
        }
    }
    // calculate left and right entropy
    let left_entropy = calculate_entropy(&left_labels);
    let right_entropy = calculate_entropy(&right_labels);

    let branch_entropy = left_data.len() as f64 / n as f64 * left_entropy
        + right_data.len() as f64 / n as f64 * right_entropy;
    let inf_gain = parent_entropy - branch_entropy;

    (inf_gain, left_data, left_labels, right_data, right_labels)
}

fn find_best_split(
    m_X_train: &Vec<Vec<u32>>,
    y_train: &Vec<u32>,
) -> (
    usize,
    f64,
    f64,
    Vec<Vec<u32>>,
    Vec<u32>,
    Vec<Vec<u32>>,
    Vec<u32>,
) {
    let mut max_gain = -1f64;
    let mut best_feature_idx: Option<usize> = None;
    let mut best_threshold: Option<f64> = None;
    let mut best_left_data: Vec<Vec<u32>> = vec![];
    let mut best_left_labels: Vec<u32> = vec![];
    let mut best_right_data: Vec<Vec<u32>> = vec![];
    let mut best_right_labels: Vec<u32> = vec![];

    let n_features = m_X_train[0].len();
    for feature_idx in 0..n_features {
        let mut values = m_X_train
            .iter()
            .map(|x| x[feature_idx])
            .collect::<Vec<u32>>();
        values.sort_by(|a, b| a.cmp(b));
        // let value_curr_next = values.iter().zip(values.iter().skip(1));
        for pair in values.windows(2) {
            let threshold = (pair[0] + pair[1]) as f64 / 2.0;
            let (inf_gain, left_data, left_labels, right_data, right_labels) =
                calculate_infomation_gain(m_X_train, y_train, feature_idx, threshold);
            if inf_gain > max_gain {
                max_gain = inf_gain;
                best_feature_idx = Some(feature_idx);
                best_threshold = Some(threshold);
                best_left_data = left_data;
                best_left_labels = left_labels;
                best_right_data = right_data;
                best_right_labels = right_labels;
            }
        }
    }

    (
        best_feature_idx.unwrap(),
        best_threshold.unwrap(),
        max_gain,
        best_left_data,
        best_left_labels,
        best_right_data,
        best_right_labels,
    )
}

fn build_tree(
    m_X_train: &Vec<Vec<u32>>,
    y_train: &Vec<u32>,
    curr_depth: u32,
    max_depth: u32,
) -> Node {
    let mut labels_set: HashSet<u32> = y_train.iter().map(|x| *x).collect::<HashSet<u32>>();
    if labels_set.len() == 1 {
        return Node {
            feature_idx: None,
            threshold: None,
            value: Some(1.0),
            left: None,
            right: None,
        };
    }
    if curr_depth >= max_depth {
        let label_set_iter = labels_set.iter();
        let majority_class = label_set_iter
            .max_by_key(|&x| labels_set.iter().filter(|&y| *y == *x).count())
            .unwrap();
        return Node {
            feature_idx: None,
            threshold: None,
            value: Some(majority_class.as_f64()),
            left: None,
            right: None,
        };
    }

    let (feature_idx, threshold, max_gain, left_data, left_labels, right_data, right_labels) =
        find_best_split(m_X_train, y_train);

    // no gain
    if max_gain <= 0.0 {
        let label_set_iter = labels_set.iter();
        let majority_class = label_set_iter
            .max_by_key(|&x| labels_set.iter().filter(|&y| *y == *x).count())
            .unwrap();
        return Node {
            feature_idx: None,
            threshold: None,
            value: Some(majority_class.as_f64()),
            left: None,
            right: None,
        };
    }

    // gain
    let left_node = build_tree(&left_data, &left_labels, curr_depth + 1, max_depth);
    let right_node = build_tree(&right_data, &right_labels, curr_depth + 1, max_depth);

    Node {
        feature_idx: Some(feature_idx),
        threshold: Some(threshold),
        value: None,
        left: Some(Box::new(left_node)),
        right: Some(Box::new(right_node)),
    }
}

fn predict(tree: &Option<Box<Node>>, sample: &Vec<u32>) -> f64 {

    if let Some(tree) = tree {
        if tree.value.is_some() {
            return tree.value.unwrap();
        }
        if sample[tree.feature_idx.unwrap()] as f64 <= tree.threshold.unwrap() {
            return predict(&tree.left, sample);
        } else {
            return predict(&tree.right, sample);
        }
    }

    -1.0
}
fn train_test_split(
    mX_data: &Vec<Vec<u32>>,
    y_data: &Vec<u32>,
    test_size: f64,
) -> (Vec<Vec<u32>>, Vec<u32>, Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = rand::rng();
    let n = mX_data.len();
    let n_test = (n as f64 * test_size) as usize;
    let mXy_data = mX_data
        .into_iter()
        .zip(y_data.iter())
        .collect::<Vec<(&Vec<u32>, &u32)>>();
    let rand_test_indices = mXy_data
        .iter()
        .choose_multiple(&mut rng, n_test)
        .into_iter()
        .collect::<Vec<&(&Vec<u32>, &u32)>>();
    let xy_train = mXy_data
        .iter()
        .filter(|x| !rand_test_indices.contains(x))
        .map(|x| x.to_owned())
        .collect::<Vec<(&Vec<u32>, &u32)>>();
    let xy_test = mXy_data
        .iter()
        .filter(|x| rand_test_indices.contains(x))
        .map(|x| x.to_owned())
        .collect::<Vec<(&Vec<u32>, &u32)>>();

    (
        xy_train
            .iter()
            .map(|x| x.0.to_owned())
            .collect::<Vec<Vec<u32>>>(),
        xy_train
            .iter()
            .map(|x| x.1.to_owned())
            .collect::<Vec<u32>>(),
        xy_test
            .iter()
            .map(|x| x.0.to_owned())
            .collect::<Vec<Vec<u32>>>(),
        xy_test.iter().map(|x| x.1.to_owned()).collect::<Vec<u32>>(),
    )
}

fn decision_tree() -> Result<(), Box<dyn Error>> {
    init_logger();
    const FILE_NAME: &str = "data/email/spam.csv";
    const TOP_N: usize = 2000;
    // read csv file
    let rows: Vec<Vec<String>> = read_from_csv(FILE_NAME)?;
    // info!("Rows{}:\n{:?}", TOP_N, &rows[0..5]);

    // extract keywords
    let keywords = extract_keywords(&rows, 2000)?;
    // info!("Keywords {}:\n{:?}", TOP_N, &keywords[0..TOP_N]);

    // preprocess data
    let (m_X_train, y_train) = preprocess_data(&rows, &keywords)?;
    // info!("X_train {:?}", &X_train[0]);
    // info!("y_train {:?}", &y_train[1]);

    // train test split
    let (m_X_train, y_train, m_X_test, y_test) = train_test_split(&X_train, &y_train, 0.2);
    // info!("X_train:{:?}, y_train:{:?}", X_train.len(), y_train.len());
    // info!("X_test:{:?}, y_test:{:?}", X_test.len(), y_test.len());

    // build decision tree
    let tree = build_tree(&m_X_train, &y_train, 0, 10);

    // test
    let opt_tree = Some(Box::new(tree));

    // predict and test

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::decision_tree::decision_tree;
    use log::info;
    use std::collections::HashSet;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_decision_tree() {
        _ = decision_tree();
    }

    #[test]
    fn test_min_max_scale() {
        init_logger();
        let mut set = HashSet::new();
        set.insert("apple");
        set.insert("banana");
        set.insert("cherry");

        let max = set.into_iter().max_by_key(|&x| x.len()).unwrap();

        info!("max: {}", max);
    }
}
