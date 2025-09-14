use core::f64;
use log::info;
use maplit::hashmap;
use rand;
use rand::rngs::StdRng;
use rand::seq::index::sample;
use rand::{Rng, SeedableRng};
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use utils::log::configuration::init_logger;

#[derive(Debug, Clone, Default)]
struct Node {
    feature_idx: Option<usize>,
    threshold: Option<f64>,
    value: Option<u32>,
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
    let mut features: Vec<u32> = vec![];

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
    let mut m_x_data: Vec<Vec<u32>> = vec![];
    let mut y_data: Vec<u32> = vec![];
    for row in data {
        let label = if "spam" == row[0] { 1 } else { 0 };
        let features = extract_features(&row[1], keywords)?;
        m_x_data.push(features);
        y_data.push(label);
    }

    Ok((m_x_data, y_data))
}
fn calculate_entropy(labels: &Vec<u32>) -> f64 {
    let len_n = labels.len();
    if len_n == 0 {
        return 0.0;
    }
    let unq_labels = labels.iter().map(|&x| x).collect::<HashSet<u32>>();
    let mut entropy = 0.0;
    for label in unq_labels {
        let label_count = labels.iter().filter(|&&x| x == label).count() as f64;
        let p = label_count / len_n as f64;
        entropy -= p * p.log2();
    }

    entropy
}

fn calculate_infomation_gain(
    m_x_train: &Vec<Vec<u32>>,
    y_train: &Vec<u32>,
    feature_idx: usize,
    threshold: f64,
) -> (f64, Vec<Vec<u32>>, Vec<u32>, Vec<Vec<u32>>, Vec<u32>) {
    let n = m_x_train.len();
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
    // info!("parent_entropy: {} at feature_idx:{}", parent_entropy, feature_idx);
    // split left and right by entropy
    for (data_row, label_row) in m_x_train.iter().zip(y_train.iter()) {
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
    m_x_train: &Vec<Vec<u32>>,
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
    let mut max_gain = -1.0;
    let mut best_feature_idx: Option<usize> = None;
    let mut best_threshold: Option<f64> = None;
    let mut best_left_data: Vec<Vec<u32>> = vec![];
    let mut best_left_labels: Vec<u32> = vec![];
    let mut best_right_data: Vec<Vec<u32>> = vec![];
    let mut best_right_labels: Vec<u32> = vec![];

    let n_features = m_x_train[0].len();
    for feature_idx in 0..n_features {
        let feature_idx_set = m_x_train
            .iter()
            .map(|x| x[feature_idx])
            .collect::<HashSet<u32>>();
        let mut values = feature_idx_set.iter().collect::<Vec<&u32>>();
        // values.sort();
        // values.sort_by_key(|&&x| std::cmp::Reverse(x));
        values.sort_by_key(|&&x| x);
        // let value_curr_next = values.iter().zip(values.iter().skip(1));
        // info!("values.len:{}", values.len());
        for pair in values.windows(2) {
            let threshold = (pair[0] + pair[1]) as f64 / 2.0;
            let (inf_gain, left_data, left_labels, right_data, right_labels) =
                calculate_infomation_gain(m_x_train, y_train, feature_idx, threshold);
            // info!("feature_idx:{}, threshold:{}, inf_gain:{}, pair:{:?}", feature_idx, threshold, inf_gain, pair);
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
    m_x_train: &Vec<Vec<u32>>,
    y_train: &Vec<u32>,
    curr_depth: u32,
    max_depth: u32,
) -> Node {
    let mut labels_set = y_train.iter().map(|x| *x).collect::<HashSet<u32>>();
    // info!("labels_set {:?}", labels_set);
    if labels_set.len() == 1 {
        // info!("labels_set {:?}", labels_set);
        return Node {
            feature_idx: None,
            threshold: None,
            value: Some(y_train[0].to_owned()),
            left: None,
            right: None,
        };
    }
    let label_set_iter = labels_set.iter();
    if curr_depth >= max_depth {
        //
        let majority = label_set_iter
            .max_by_key(|&x| y_train.iter().filter(|&y| y == x).count())
            .unwrap()
            .to_owned();
        // info!("majority_class {:?}", majority_class);
        return Node {
            feature_idx: None,
            threshold: None,
            value: Some(majority),
            left: None,
            right: None,
        };
    }
    // info!("labels_set {:?}", labels_set);
    let (feature_idx, threshold, max_gain, left_data, left_labels, right_data, right_labels) =
        find_best_split(m_x_train, y_train);
    // info!("feature_idx {:?}, threshold:{}, max_gain:{}", feature_idx, threshold, max_gain);
    // no gain
    if max_gain <= 0.0 {
        // let label_set_iter = labels_set.iter();
        let majority = label_set_iter
            .max_by_key(|&x| y_train.iter().filter(|&y| y == x).count())
            .unwrap()
            .to_owned();
        // info!("majority_class {:?}", majority_class);
        return Node {
            feature_idx: None,
            threshold: None,
            value: Some(majority),
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

fn predict(tree: &Option<Box<Node>>, sample: &Vec<u32>) -> u32 {
    if let Some(tree) = tree {
        if let Some(tree_val) = tree.value {
            return tree_val;
        }
        if sample[tree.feature_idx.unwrap()] as f64 <= tree.threshold.unwrap() {
            return predict(&tree.left, sample);
        } else {
            return predict(&tree.right, sample);
        }
    }
    2
}
fn train_test_split(
    m_x_data: &Vec<Vec<u32>>,
    y_data: &Vec<u32>,
    test_size: f64,
) -> (Vec<Vec<u32>>, Vec<u32>, Vec<Vec<u32>>, Vec<u32>) {
    let mut rng = StdRng::seed_from_u64(48);
    // let mut rng = rand::rng();
    let n = m_x_data.len();
    let n_test = (n as f64 * test_size) as usize;
    let m_xy_data = m_x_data
        .iter()
        .zip(y_data.iter())
        .collect::<Vec<(&Vec<u32>, &u32)>>();
    let rand_test_indices = sample(&mut rng, n, n_test)
        .iter()
        .collect::<HashSet<usize>>();
    let mut xy_train: Vec<(&Vec<u32>, &u32)> = vec![];
    let mut xy_test: Vec<(&Vec<u32>, &u32)> = vec![];
    for i in 0..n {
        if !rand_test_indices.contains(&i) {
            xy_train.push(m_xy_data[i]);
        } else {
            xy_test.push(m_xy_data[i]);
        }
    }

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

fn print_tree(keywords: &Vec<String>, node: &Option<Box<Node>>, indent: &str) {
    if let Some(node) = node {
        if node.value.is_some() {
            info!("Leaf: {}", node.value.unwrap());
        } else {
            info!(
                "{}Feature '{}' count <= {}",
                indent,
                keywords[node.feature_idx.unwrap()],
                node.threshold.unwrap()
            );
            print_tree(keywords, &node.left, format!("{}L:", indent).as_str());
            print_tree(keywords, &node.right, format!("{}R:", indent).as_str());
        }
    }
}

fn make_decision_tree() -> Result<(), Box<dyn Error>> {
    init_logger();
    const FILE_NAME: &str = "data/email/spam.csv";
    const TOP_N: u32 = 200;
    const MAX_DEPTH: u32 = 3;
    // read csv file
    let rows: Vec<Vec<String>> = read_from_csv(FILE_NAME)?;
    // info!("Rows{}:\n{:?}", TOP_N, &rows[0..5]);

    // extract keywords
    let keywords = extract_keywords(&rows, TOP_N)?;
    // info!("Keywords {}:\n{:?}", TOP_N, &keywords[0..TOP_N]);

    // preprocess data
    let (m_x_data, y_data) = preprocess_data(&rows, &keywords)?;
    // info!("X_train {:?}", &X_train[0]);
    // info!("y_train {:?}", &y_train[1]);

    // train tests split
    let (m_x_train, y_train, m_x_test, y_test) = train_test_split(&m_x_data, &y_data, 0.2);
    info!("X_train:{:?}, y_train:{:?}", m_x_train.len(), y_train.len());
    info!("X_test:{:?}, y_test:{:?}", m_x_test.len(), y_test.len());

    // build decision tree
    let start = Instant::now();
    let tree = build_tree(&m_x_train, &y_train, 0, MAX_DEPTH);
    let end = Instant::now();
    let tree_some = Some(Box::new(tree.clone()));
    info!(
        "Build tree cost time: {:.3} sec",
        (end - start).as_secs_f64()
    );
    print_tree(&keywords, &tree_some, "");
    // tests
    // let opt_tree = Some(Box::new(tree));

    // tests
    let mut correct_count = 0u32;
    for (sample, label) in m_x_test.iter().zip(y_test.iter()) {
        let pred = predict(&tree_some, sample);
        if pred == *label {
            correct_count += 1;
        }
    }
    let accuracy = correct_count as f64 / m_x_test.len() as f64;
    info!("Accuracy: {:.2}", accuracy * 100.0);
    // predict
    let new_msg = "Free tickets to win a prize! Call now!";
    let new_sample = extract_features(&new_msg.to_string(), &keywords)?;
    let prediction = predict(&tree_some, &new_sample);
    info!(
        "Prediction: {}: {}",
        new_msg,
        if prediction == 1 { "spam" } else { "ham" }
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::decision_tree::make_decision_tree;
    use log::info;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_decision_tree() {
        _ = make_decision_tree();
    }

    #[test]
    fn test_min_max_scale() {
        init_logger();
        let n_features = 10;
        for feature_idx in 0..n_features {
            info!("feature_idx:{}", feature_idx);
        }
    }
}
