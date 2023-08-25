use std::cmp::Ord;
use std::collections::HashMap;

// Define the SortByKey trait
trait SortByKey<K: Ord, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)>;
}

// Implement SortByKey for HashMap
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)> {
        let mut sorted_pairs: Vec<(&K, &V)> = self.iter().collect();
        sorted_pairs.sort_by_key(|&(k, _)| k); // Specify lifetimes for references
        sorted_pairs
    }
}

fn main() {
    // Create a new HashMap with key-value pairs
    let mut my_map = HashMap::new();
    my_map.insert(3, "Three");
    my_map.insert(1, "One");
    my_map.insert(2, "Two");
    my_map.insert(4, "Four");

    // Print the original map
    println!("Original HashMap: {:?}", my_map);

    // Sort and print the map by key in ascending order using SortByKey trait
    let sorted_map = my_map.sort_by_key();
    println!("Sorted HashMap: {:?}", sorted_map);
}
