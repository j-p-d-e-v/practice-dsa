use std::collections::HashMap;
use std::time::Instant;
fn check_if_anagram_v1(input_1: &str, input_2: &str) -> bool {
    if input_1.len() != input_2.len() {
        return false;
    }
    let mut input_1_chars: HashMap<char, usize> = HashMap::new();
    let mut input_2_chars: HashMap<char, usize> = HashMap::new();

    for c in input_1.chars() {
        input_1_chars.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    for c in input_2.chars() {
        input_2_chars.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    for (k, a) in input_1_chars.into_iter() {
        if let Some(b) = input_2_chars.get_mut(&k) {
            *b -= a;
        }
    }
    input_2_chars
        .into_values()
        .filter(|v| *v > 0)
        .collect::<Vec<usize>>()
        .len()
        == 0
}
fn check_if_anagram_v2(input_1: &str, input_2: &str) -> bool {
    let mut input_1_chars: Vec<char> = input_1.chars().collect();
    let mut input_2_chars: Vec<char> = input_2.chars().collect();
    input_1_chars.sort_unstable();
    input_2_chars.sort_unstable();
    input_1_chars == input_2_chars
}

fn group_anagrams_v2(inputs: Vec<&str>, mut output: Vec<Vec<&str>>) {
    println!("{}", std::iter::repeat("=").take(20).collect::<String>());
    println!("Input: {:?}", inputs);
    println!("Expected:{:?}", output);
    let timer: Instant = Instant::now();
    let mut groups: HashMap<String, Vec<&str>> = HashMap::new();

    for input in inputs {
        let mut input_chars: Vec<char> = input.chars().collect();
        input_chars.sort_unstable();
        let key: String = input_chars.into_iter().collect::<String>();
        groups.entry(key).or_default().push(&input);
    }

    let mut result: Vec<Vec<&str>> = groups
        .into_values()
        .map(|mut items| {
            items.sort_unstable();
            items
        })
        .collect();
    let mut output: Vec<Vec<&str>> = output
        .into_iter()
        .map(|mut items| {
            items.sort_unstable();
            items
        })
        .collect();
    result.sort_unstable();
    output.sort_unstable();
    assert_eq!(result, output);
    println!("Got: {:?}", result);
    println!("v2 completed at: {}", timer.elapsed().as_micros());
}
fn group_anagrams_v1(inputs: Vec<&str>, mut output: Vec<Vec<&str>>) {
    println!("{}", std::iter::repeat("=").take(20).collect::<String>());
    println!("Input: {:?}", inputs);
    println!("Expected:{:?}", output);
    let timer: Instant = Instant::now();
    let mut result: Vec<Vec<&str>> = Vec::new();
    let mut groups: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut already_grouped: Vec<&str> = Vec::new();
    for a in 0..inputs.len() {
        let b_start = a + 1;
        let input_1 = inputs.get(a).unwrap();
        let mut has_anagram = false;
        if already_grouped.contains(&input_1) {
            continue;
        }
        for b in b_start..inputs.len() {
            let input_2 = inputs.get(b).unwrap();
            if check_if_anagram_v1(input_1, input_2) {
                groups
                    .entry(input_1)
                    .and_modify(|l| l.push(input_2))
                    .or_insert(vec![input_2]);
                already_grouped.push(input_2);
                has_anagram = true;
            }
        }
        if !has_anagram {
            groups.insert(input_1, vec![]);
        }
    }
    for (key, mut values) in groups.into_iter() {
        let mut grouped: Vec<&str> = Vec::from(vec![key]);
        grouped.append(&mut values);
        grouped.sort();
        result.push(grouped);
    }
    for items in &mut output {
        items.sort();
    }
    result.sort();
    output.sort();
    assert_eq!(result, output);
    println!("Got: {:?}", result);
    println!("v1 completed at: {}", timer.elapsed().as_micros());
}

// Group Anagrams
fn main() -> Result<(), String> {
    let strs: Vec<&str> = vec!["act", "pots", "tops", "cat", "stop", "hat"];
    let output: Vec<Vec<&str>> = vec![
        vec!["hat"],
        vec!["act", "cat"],
        vec!["stop", "pots", "tops"],
    ];
    group_anagrams_v1(strs.clone(), output.clone());
    group_anagrams_v2(strs.clone(), output.clone());

    let strs: Vec<&str> = vec!["x"];
    let output: Vec<Vec<&str>> = vec![vec!["x"]];
    group_anagrams_v1(strs.clone(), output.clone());
    group_anagrams_v2(strs.clone(), output.clone());

    let strs: Vec<&str> = vec![""];
    let output: Vec<Vec<&str>> = vec![vec![""]];
    group_anagrams_v1(strs.clone(), output.clone());
    group_anagrams_v2(strs.clone(), output.clone());

    let strs: Vec<&str> = vec![
        "python", "listen", "silent", "enlist", "inlets", "evil", "live", "veil", "vile", "rat",
        "tar", "art", "loop", "pool", "polo", "dust", "stud", "arc", "car", "java", "kotlin",
        "rust", "swift",
    ];
    let output: Vec<Vec<&str>> = vec![
        // singletons
        vec!["python"],
        vec!["java"],
        vec!["kotlin"],
        vec!["rust"],
        vec!["swift"],
        // pairs
        vec!["dust", "stud"],
        vec!["arc", "car"],
        // triples
        vec!["rat", "tar", "art"],
        vec!["loop", "pool", "polo"],
        // quads
        vec!["listen", "silent", "enlist", "inlets"],
        vec!["evil", "live", "veil", "vile"],
    ];
    group_anagrams_v1(strs.clone(), output.clone());
    group_anagrams_v2(strs.clone(), output.clone());
    Ok(())
}
