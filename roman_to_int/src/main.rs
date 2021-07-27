use std::collections::HashMap;
#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut mapping = HashMap::new();
        mapping.insert("I", 1);
        mapping.insert("V", 5);
        mapping.insert("X", 10);
        mapping.insert("L", 50);
        mapping.insert("C", 100);
        mapping.insert("D", 500);
        mapping.insert("M", 1000);
        let mut total = 0;
        let characters: Vec<char> = s.chars().collect();
        let mut vec_map = Vec::new();
        let mut index = 0;
        for i in characters.iter() {
            let value = mapping[&*i.to_string()];
            if index == 0 {
                vec_map.push(value);
                index = index + 1;
            } else if index != 0 {
                if vec_map[index - 1] < value {
                    vec_map.push(value - vec_map[index - 1]);
                    vec_map.remove(index - 1);
                    index = vec_map.len()
                } else {
                    vec_map.push(value);
                    index = index + 1;
                }
            }
        }
        for j in vec_map.iter() {
            total = total + j;
        }
        total
    }
}
fn main() {
    assert_eq!(150, Solution::roman_to_int("LLL".to_string()));
    assert_eq!(4, Solution::roman_to_int("IV".to_string()));
    assert_eq!(9, Solution::roman_to_int("IX".to_string()));
    assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
}
