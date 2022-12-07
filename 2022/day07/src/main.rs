use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should be able to read file");
    let mut cwd: Vec<&str> = vec![""];
    let mut dir_sizes = HashMap::<String, i32>::new();
    for line in contents.lines() {
        let args: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        if args[0] == "$" && args[1] == "cd" {
            match args[2] {
                "/" => cwd = vec![""],
                ".." => _ = cwd.pop(),
                dir => _ = cwd.push(dir),
            }
        }
        match args[0].parse::<i32>() {
            Ok(n) => {
                for i in 0..cwd.len() {
                    dir_sizes
                        .entry(cwd[0..cwd.len() - i].join("/"))
                        .and_modify(|s| *s += n)
                        .or_insert(n);
                }
            }
            Err(_) => {}
        }
    }

    let sum_dir_below_100000: i32 = dir_sizes
        .clone()
        .into_values()
        .filter(|s| *s <= 100000)
        .sum();
    println!(
        "Sum of all directories above 100000: {}",
        sum_dir_below_100000
    );
    let space_needed: i32 = dir_sizes[""] - 70000000 + 30000000;
    let smallest_dir_to_delete: i32 = dir_sizes
        .into_values()
        .filter(|s| *s >= space_needed)
        .min()
        .unwrap();
    println!("Size of smallest dir to delete: {}", smallest_dir_to_delete);
}
