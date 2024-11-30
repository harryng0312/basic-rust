use std::{collections::HashMap};
use std::fmt::Debug;
use std::io::Write;
type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::basic_collection::{show, Table};

    #[test]
    fn test_hashmap() {
        let mut table = Table::new();
        table.insert(
            "Gesualdo".to_string(),
            vec![
                "many madrigals".to_string(),
                "Tenebrae Responsoria".to_string(),
            ],
        );
        table.insert(
            "Caravaggio".to_string(),
            vec![
                "The Musicians".to_string(),
                "The Calling of St. Matthew".to_string(),
            ],
        );
        table.insert(
            "Cellini".to_string(),
            vec![
                "Perseus with the head of Medusa".to_string(),
                "a salt cellar".to_string(),
            ],
        );
        show(&table);
        assert_eq!(table["Gesualdo"][0], "many madrigals");
    }

    #[test]
    fn test_vector() {
        let mut ls_str = Vec::<String>::new();
        ls_str.push(String::from("test1"));
        let itm = &ls_str[0];
        std::io::stdout().write_all(format!("{}", itm).as_bytes()).expect("TODO: panic message");
    }
}
