// https://leetcode.com/problems/valid-anagram/description/

// Problem:
// Given two strings s and t, return true if t is an anagram of s, 
// and false otherwise.

// An Anagram is a word or phrase formed by 
// rearranging the letters of a different word or 
// phrase, typically using all the original letters exactly once.

use std::collections::HashMap;

fn main() {}

// Time complexity: O(n) where n = s.len() + t.len()
// Space complexity: O(n) where n = s.len() + t.len()
fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_hashmap: HashMap<char, u8> = HashMap::new();
    let mut t_hashmap: HashMap<char, u8> = HashMap::new();

    for c in s.chars() {
        *s_hashmap.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        *t_hashmap.entry(c).or_insert(0) += 1;
    }

    for (key, value) in s_hashmap.into_iter() {
        if t_hashmap.contains_key(&key) {
            if t_hashmap.get(&key).unwrap() != &value {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

// Time complexity: O(nlogn) where n = s.len() + t.len()
// Space complexity: O(n) where n = s.len() + t.len()
fn another_is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_arr: Vec<char> = s.chars().collect();
    let mut t_arr: Vec<char> = t.chars().collect();

    s_arr.sort();
    t_arr.sort();

    let s_sorted: String = s_arr.into_iter().collect();
    let t_sorted: String = t_arr.into_iter().collect();

    if s_sorted != t_sorted {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_anagram;
    use crate::another_is_anagram;

    #[test]
    fn test_is_anagram() {
        let s1 = "anagram";
        let t1 = "nagaram";

        let s2 = "rat";
        let t2 = "cat";

         assert_eq!(is_anagram(s1, t1), true);
         assert_eq!(is_anagram(s2, t2), false);
    }

    #[test]
    fn test_other_is_anagram() {
        let s1 = "anagram";
        let t1 = "nagaram";

        let s2 = "rat";
        let t2 = "cat";

         assert_eq!(another_is_anagram(s1, t1), true);
         assert_eq!(another_is_anagram(s2, t2), false);
    }
}
