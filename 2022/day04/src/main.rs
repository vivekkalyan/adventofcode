use std::fs;
fn main() {
    let contents = fs::read_to_string("input").expect("Should be able to read file");
    let mut num_contain = 0;
    let mut num_overlap = 0;
    for line in contents.lines() {
        let splits: Vec<u8> = line
            .splitn(4, [',', '-'])
            .map(|s| s.parse().unwrap())
            .collect();
        let start1 = splits[0];
        let end1 = splits[1];
        let start2 = splits[2];
        let end2 = splits[3];

        if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
            num_contain += 1;
        }

        if start1 <= end2 && start2 <= end1 {
            num_overlap += 1;
        }
    }
    println!("Number of pairs which contain each other: {}", num_contain);
    println!("Number of pairs which overlap: {}", num_overlap);
}
