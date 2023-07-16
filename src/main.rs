// cargo run -- --source1 data1.csv --source2 data2.csv --primarykeys pk1,pk2

use clap::Parser;
use csv;
use std::{collections::HashMap, hash::Hash};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to the first csv file
    #[arg(long)]
    source1: String,
    /// Path to the second csv file
    #[arg(long)]
    source2: String,
    /// Primary keys
    #[arg(long, value_delimiter = ',', num_args = 1..)]
    primarykeys: Vec<String>,
}

fn get_mapping_from_csv(path: &String) -> HashMap<String, usize> {
    let mut mapping = HashMap::new();
    let mut reader = csv::Reader::from_path(path).expect("Could not read csv file.");

    if let Ok(record) = reader.headers() {
        for (idx, value) in record.into_iter().enumerate() {
            mapping.insert(value.to_string(), idx);
        }
    }
    mapping
}

/// Reads csv file into a collection by extracting key value pairs. Note that the indizes of the key columns
/// and the value columns have to be specified.
pub fn read_csv_into_map(
    path: &String,
    key_cols: Vec<&usize>,
    val_cols: Vec<&usize>,
) -> HashMap<Vec<String>, Vec<String>> {
    let mut data_store = HashMap::new();
    let mut reader = csv::Reader::from_path(path).expect("Could not read csv file.");

    for record in reader.records() {
        let record = record.unwrap();
        let key: Vec<String> = key_cols.iter().map(|&x| record[*x].to_string()).collect();
        let value: Vec<String> = val_cols.iter().map(|&x| record[*x].to_string()).collect();
        data_store.insert(key, value);
    }

    data_store
}

fn keys_match<T: Eq + Hash, U, V>(map1: &HashMap<T, U>, map2: &HashMap<T, V>) -> bool {
    map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
}

/// Collects the indizes of the columns from the mappings
fn get_col_indizes<'a>(
    mapping: &'a HashMap<String, usize>,
    columns: &Vec<&String>,
) -> Vec<&'a usize> {
    let cols = columns.iter().map(|&x| mapping.get(x).unwrap()).collect();

    cols
}

fn main() {
    let args = CliArgs::parse();

    let map1 = get_mapping_from_csv(&args.source1);
    let map2 = get_mapping_from_csv(&args.source2);

    // Perform sanity checks:
    // 1. Compare headers
    if !keys_match(&map1, &map2) {
        panic!("Headers in csv files differ!")
    }

    // 2. Check presence of primary_keys
    if !args.primarykeys.iter().all(|k| map1.contains_key(k)) {
        panic!("First csv file does not contain specified primary keys.")
    }
    if !args.primarykeys.iter().all(|k| map2.contains_key(k)) {
        panic!("First csv file does not contain specified primary keys.")
    }

    let key_cols: Vec<&String> = args.primarykeys.iter().collect();
    let val_cols: Vec<&String> = map1.keys().filter(|&x| !key_cols.contains(&x)).collect();

    let key_cols1 = get_col_indizes(&map1, &key_cols);
    let val_cols1 = get_col_indizes(&map1, &val_cols);

    let key_cols2 = get_col_indizes(&map2, &key_cols);
    let val_cols2 = get_col_indizes(&map2, &val_cols);

    let map_1 = read_csv_into_map(&args.source1, key_cols1, val_cols1);
    let map_2 = read_csv_into_map(&args.source2, key_cols2, val_cols2);

    
    let only_map_1: Vec<_> = map_1.keys().filter(|&x| !map_2.contains_key(x)).collect();
    for key in only_map_1 {
        println!("The key {:?} is in only present in file 1.", key)
    }
    
    /*
    let only_map_2: Vec<_> = map_2.keys().filter(|&x| !map_1.contains_key(x)).collect();
    for key in only_map_2 {
        println!("The key {:?} is in only present in file 2.", key)
    }
    */

    for key in map_1.keys().into_iter(){
        if !map_2.contains_key(key){
            println!("Key {:?} not present in second hashmap", key)
        }
        else if map_1.get(key).unwrap() != map_2.get(key).unwrap() {
            println!("Values for key {:?} differ: {:?} vs {:?}", key, map_1.get(key).unwrap(), map_2.get(key).unwrap())
        }
    }

    /*
    let both_maps: Vec<_> = map_1.keys().filter(|&x| map_2.contains_key(x)).collect();
    for key in both_maps{
        let value_1 = map_1.get(key).unwrap();  
        println!("{}", value_1);
    }
     
    */
}

#[cfg(test)]
mod tests {
    use super::keys_match;
    use std::collections::HashMap;

    #[test]
    fn test_keys_match() {
        let map1: HashMap<_, _> = HashMap::from_iter([("one", 1), ("two", 2), ("three", 3)]);
        let map2: HashMap<_, _> = HashMap::from_iter([("three", 3), ("two", 2), ("one", 1)]);
        assert_eq!(keys_match(&map1, &map2), true);

        let map1: HashMap<_, _> = HashMap::from_iter([("one", 1), ("two", 2), ("three", 3)]);
        let map2: HashMap<_, _> = HashMap::from_iter([("3", 3), ("2", 2), ("1", 1)]);
        assert_eq!(keys_match(&map1, &map2), false);
    }
}
