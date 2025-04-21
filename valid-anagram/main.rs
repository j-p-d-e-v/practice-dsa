fn is_anagram(s: String, t: String) -> bool {
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

    assert!(is_anagram(s, t) == true, "Not an anagram");

    let s: String = String::from("jar");
    let t: String = String::from("jam");

    assert!(is_anagram(s, t) == false, "Its an anagram");
}
