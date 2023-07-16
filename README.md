# CSVDIFF - EXAMPLE APPLICATION

- Run application via "cargo run -- --source1 data1.csv --source2 data2.csv --primarykeys pk1,pk2"

## Features
- Specification of primary keys possibles


## Example for Stackoverflow
use std::{
    collections::HashMap,
};

fn main(){
    let keys = vec![vec!["A".to_string(), "1".to_string()], 
        vec!["B".to_string(), "2".to_string()],
        vec!["C".to_string(), "3".to_string()]];
    
    let map_1: HashMap<Vec<String>, i32> = HashMap::from_iter(vec![(keys[0].clone(), 1), (keys[1].clone(), 2)]);  
    let map_2: HashMap<Vec<String>, i32> = HashMap::from_iter(vec![(keys[0].clone(), 2), (keys[2].clone(), 2)]);  
    
    
    for key in map_1.keys().into_iter(){
        if !map_2.contains_key(key){
            println!("Key {:?} not present in second hashmap", key)
        }
        else if map_1.get(key).unwrap() != map_2.get(key).unwrap() {
            println!("Values for key {:?} differ: {:?} vs {:?}", key, map_1.get(key).unwrap(), map_2.get(key).unwrap())
        }
    }
    
    
    let both_maps: Vec<_> = map_1.keys().filter(|&x| map_2.contains_key(x)).collect();
    for key in both_maps{
        let value_1 = map_1.get(key).unwrap();  
        println!("{}", value_1);
    }
}