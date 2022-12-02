use std::fs; 

fn main() {
    let contents = fs::read_to_string("input").expect("Should be able to read file");
    let mut total_score = 0;
    for line in contents.lines() {
        let result: Vec<_> = line.split(" ").collect();
        // X (rock), Y (paper), Z (scissors)
        // A (rock), B (paper), C (scissors)
        match result[1] {
            "X" => {
                total_score += 1;
                match result[0] {
                    "A" => total_score += 3,
                    "B" => total_score += 0,
                    "C" => total_score += 6,
                    &_ => total_score += 0,
                }
            },
            "Y" => {
                total_score += 2;
                match result[0] {
                    "A" => total_score += 6,
                    "B" => total_score += 3,
                    "C" => total_score += 0,
                    &_ => total_score += 0,
                }
            },
            "Z" => {
                total_score += 3;
                match result[0] {
                    "A" => total_score += 0,
                    "B" => total_score += 6,
                    "C" => total_score += 3,
                    &_ => total_score += 0,
                }
            },
            &_ => total_score += 0,
        }
    }
    println!("Total score assuming second column is what you should play: {:?}", total_score);

    let mut total_score2 = 0;
    for line in contents.lines() {
        let result: Vec<_> = line.split(" ").collect();
        // X (lose), Y (draw), Z (win)
        // A (rock), B (paper), C (scissors)
        match result[1] {
            "X" => {
                total_score2 += 0;
                match result[0] {
                    "A" => total_score2 += 3,
                    "B" => total_score2 += 1,
                    "C" => total_score2 += 2,
                    &_ => total_score2 += 0,
                }
            },
            "Y" => {
                total_score2 += 3;
                match result[0] {
                    "A" => total_score2 += 1,
                    "B" => total_score2 += 2,
                    "C" => total_score2 += 3,
                    &_ => total_score2 += 0,
                }
            },
            "Z" => {
                total_score2 += 6;
                match result[0] {
                    "A" => total_score2 += 2,
                    "B" => total_score2 += 3,
                    "C" => total_score2 += 1,
                    &_ => total_score2 += 0,
                }
            },
            &_ => total_score2 += 0,
        }
    }
    println!("Total score if the second column is outcome to achieve: {:?}", total_score2);
}
