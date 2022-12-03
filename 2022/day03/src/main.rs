use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input").expect("Should be able to read file");
    let mut sum = 0;
    for line in contents.lines() {
        let (first, second) = line.split_at(line.len()/2);
        println!("{} {}", first, second);
        let common_char = first.chars().find(|c| second.contains(*c)).expect("");
        println!("{}", common_char);
        let diff;
        if common_char.is_lowercase() {
            diff = 96;
        } else {
            diff = 64 - 26;
        }
        let common_int: u32 = common_char as u32 - diff;
        println!("{} {}", common_char, common_int);
        sum += common_int;
    }
    println!("Sum of common items: {}", sum);

    let mut group_sum = 0;

    for group in contents.lines().collect::<Vec<_>>().chunks(3) {
        let group1: HashSet<char> = group[0].chars().collect();
        let group2: HashSet<char> = group[1].chars().collect();
        let group3: HashSet<char> = group[2].chars().collect();

        let common: HashSet<char> = group1.intersection(&group2).cloned().collect();
        let common: HashSet<char> = common.intersection(&group3).cloned().collect();

        let common_char: char = common.iter().cloned().collect::<Vec<char>>()[0];
        let diff;
        if common_char.is_lowercase() {
            diff = 96;
        } else {
            diff = 64 - 26;
        }
        let common_int: u32 = common_char as u32 - diff;
        group_sum += common_int;
    }
    println!("Sum of group items: {}", group_sum);
}
