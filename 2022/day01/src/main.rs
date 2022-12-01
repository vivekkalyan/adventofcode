use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should be able to read file");
    let mut calories: Vec<i32> = contents
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|i| i.parse::<i32>().unwrap_or_else(|_| 0))
                .sum::<i32>()
        })
        .collect();
    calories.sort();
    calories.reverse();
    println!(
        "The elf carrying the most number of calories has is carrying {:?} calories",
        calories[0]
    );
    println!(
        "The elves carrying the 3 most number of calories has is carrying a total of {:?} calories",
        calories[0] + calories[1] + calories[2]
    );
}
