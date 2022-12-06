use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should be able to read file");
    println!("{:?}", contents);

    let mut seen_chars: Vec<char> = vec![];
    let mut num_seen_chars: i32 = 0;
    let message_lens: Vec<usize> = vec![4, 14];
    for message_len in message_lens {
        for (i, c) in contents.chars().enumerate() {
            // println!("seeing char: {} at position: {}", &c, i);
            if seen_chars.len() >= message_len {
                seen_chars.remove(0);
            }
            seen_chars.push(c);
            // println!("seen chars: {:?}", seen_chars);
            let unique_chars: HashSet<char> = seen_chars.clone().into_iter().collect();
            // println!("unique chars: {:?}", unique_chars);
            if unique_chars.len() == message_len {
                num_seen_chars = i as i32 + 1;
                break;
            }
        }
        println!(
            "For message length of {}, first message is seen at char {:?}",
            message_len, num_seen_chars
        );
    }
}
