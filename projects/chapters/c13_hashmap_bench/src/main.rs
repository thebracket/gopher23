use std::collections::HashMap;
const ELEMENTS: usize = 1_000_000;

fn main() {
    // INSERTION
    let mut my_vector = Vec::new();
    let now = std::time::Instant::now();
    for i in 0..ELEMENTS {
        my_vector.push(i);
    }
    let elapsed = now.elapsed();
    println!("Inserting {ELEMENTS} elements into a vector  took {} usecs", elapsed.as_micros());
    
    let mut my_hashmap = HashMap::new();
    let now = std::time::Instant::now();
    for i in 0..ELEMENTS {
        my_hashmap.insert(i, i);
    }
    let elapsed = now.elapsed();
    println!("Inserting {ELEMENTS} elements into a HashMap took {} usecs", elapsed.as_micros());

    // SEARCH
    let mut my_vector = Vec::new();
    for i in 0..ELEMENTS {
        my_vector.push(i);
    }

    let mut my_hashmap = HashMap::new();
    for i in 0..ELEMENTS {
        my_hashmap.insert(i, i);
    }

    // Nearly the worst case
    let element_to_find = ELEMENTS - 2;

    let now = std::time::Instant::now();
    let result = my_vector.iter().find(|n| **n == element_to_find);
    println!("{result:?}");
    let elapsed = now.elapsed();
    println!("Vector search took {} usecs", elapsed.as_micros());
    
    let now = std::time::Instant::now();
    let result = my_hashmap.get(&element_to_find);
    println!("{result:?}");
    let elapsed = now.elapsed();
    println!("HashMap search took {} usecs", elapsed.as_micros());
}
