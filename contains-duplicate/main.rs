use std::collections::HashMap;

fn main() {
    let nums: Vec<u8> = Vec::from([1, 2, 3, 3]);
    has_duplicates(nums, true);
    let nums: Vec<u8> = Vec::from([1, 2, 3, 4]);
    has_duplicates(nums, false);
}

fn has_duplicates(nums: Vec<u8>, expected: bool) {
    let mut num_counters: HashMap<u8, u8> = HashMap::new();
    for n in nums {
        match num_counters.get_mut(&n) {
            Some(value) => {
                *value += 1;
            }
            None => {
                num_counters.insert(n, 1);
            }
        }
    }
    let found_duplicates: bool = num_counters.iter().find(|item| *item.1 >= 2).is_some();
    println!(
        "Has Duplicate, Expected: {}, Got: {}",
        expected, found_duplicates
    );
    if found_duplicates != expected {
        assert!(false, "Failed");
    }
}
