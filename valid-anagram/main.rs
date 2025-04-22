use std::collections::HashMap;
use std::time::Instant;

// V2 - Slower than V1 for smaller inputs
// V2 - Faster than V1 for larger inputs
// o(s + t)
fn is_anagram_v2(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut a: HashMap<char, isize> = HashMap::new();
    let mut b: HashMap<char, isize> = HashMap::new();
    for i in s.chars() {
        a.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    for i in t.chars() {
        b.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    for (s_k, s_v) in a.iter_mut() {
        if let Some(t_v) = b.get_mut(s_k) {
            *t_v -= *s_v;
            *s_v = 0;
        }
    }
    a.into_values().sum::<isize>() == 0 && b.into_values().sum::<isize>() == 0
}

// V1 - good for smaller inputs
// V1 - O(n^2)
fn is_anagram_v1(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut b: Vec<char> = t.chars().map(|v| v.to_owned()).collect();
    for a in s.chars() {
        let mut already_found: bool = false;
        b = b
            .iter()
            .filter(|s| {
                if **s == a && !already_found {
                    already_found = true;
                    return false;
                }
                return true;
            })
            .map(|v| v.to_owned())
            .collect::<Vec<char>>();
    }

    b.len() == 0
}

fn main() {
    let s: String = String::from("racecar");
    let t: String = String::from("carrace");
    let timer = Instant::now();
    assert!(
        is_anagram_v1(s.clone(), t.clone()) == true,
        "Not an anagram"
    );
    println!("V1 Completed at: {}", timer.elapsed().as_nanos());

    let timer = Instant::now();
    assert!(
        is_anagram_v2(s.clone(), t.clone()) == true,
        "Not an anagram"
    );
    println!("V2 Completed at: {}", timer.elapsed().as_nanos());

    let s: String = String::from("jar");
    let t: String = String::from("jam");
    println!("=================================================");

    let timer = Instant::now();
    assert!(
        is_anagram_v1(s.clone(), t.clone()) == false,
        "Its an anagram"
    );
    println!("V1 Completed at: {}", timer.elapsed().as_nanos());

    let timer = Instant::now();
    assert!(
        is_anagram_v2(s.clone(), t.clone()) == false,
        "Its an anagram"
    );
    println!("V2 Completed at: {}", timer.elapsed().as_nanos());

    let s = "abcdefghij".repeat(1000);
    let t = "jihgfedcba".repeat(1000);
    println!("=================================================");
    let timer = Instant::now();
    assert!(
        is_anagram_v1(s.clone(), t.clone()) == true,
        "Its an anagram"
    );
    println!("V1 Completed at: {}", timer.elapsed().as_nanos());

    let timer = Instant::now();
    assert!(
        is_anagram_v2(s.clone(), t.clone()) == true,
        "Its an anagram"
    );
    println!("V2 Completed at: {}", timer.elapsed().as_nanos());

    let s = "abbdefghij".repeat(10000);
    let t = "xihgfedcba".repeat(10000);
    println!("=================================================");
    let timer = Instant::now();
    assert!(
        is_anagram_v1(s.clone(), t.clone()) == false,
        "Its not an anagram"
    );
    println!(
        "V1 Completed at: {}(ns) {}(ms)",
        timer.elapsed().as_nanos(),
        timer.elapsed().as_millis()
    );

    let timer = Instant::now();
    assert!(
        is_anagram_v2(s.clone(), t.clone()) == false,
        "Its not an anagram"
    );
    println!(
        "V2 Completed at: {}(ns) {}(ms)",
        timer.elapsed().as_nanos(),
        timer.elapsed().as_millis()
    );
}
