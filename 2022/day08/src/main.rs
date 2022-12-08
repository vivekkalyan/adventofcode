use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should be able to read file");
    println!("{:?}", contents);
    let len_row = contents.lines().next().unwrap().len();
    let mut is_visible: Vec<Vec<i32>> = (0..len_row)
        .map(|_| vec![0 as i32].repeat(len_row))
        .collect();
    let mut scenic_scores: Vec<Vec<i32>> = (0..len_row)
        .map(|_| vec![1 as i32].repeat(len_row))
        .collect();

    for (r, line) in contents.lines().enumerate() {
        let mut row: Vec<i32> = line.chars().map(|c| c as i32).collect();
        for c in check_visible(&row) {
            is_visible[r][c] = 1;
        }
        for (c, score) in check_scenic_scores(&row).iter().enumerate() {
            scenic_scores[r][c] *= *score as i32;
        }
        row.reverse();
        for (c, score) in check_scenic_scores(&row).iter().rev().enumerate() {
            scenic_scores[r][c] *= *score as i32;
        }
    }
    let len_row = contents.lines().next().unwrap().len();
    for c in 0..len_row {
        let mut column: Vec<i32> = contents
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .map(|chars| chars[c] as i32)
            .collect();
        for r in check_visible(&column) {
            is_visible[r][c] = 1;
        }
        for (r, score) in check_scenic_scores(&column).iter().enumerate() {
            scenic_scores[r][c] *= *score as i32;
        }
        column.reverse();
        for (r, score) in check_scenic_scores(&column).iter().rev().enumerate() {
            scenic_scores[r][c] *= *score as i32;
        }
    }
    println!(
        "Number of visible trees: {:?}",
        is_visible
            .into_iter()
            .flat_map(|x| x)
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>()
    );
    println!(
        "Max scenic score: {:?}",
        scenic_scores
            .into_iter()
            .flat_map(|x| x)
            .collect::<Vec<i32>>()
            .iter()
            .max()
            .unwrap()
    );
}

fn check_visible(row: &Vec<i32>) -> Vec<usize> {
    let mut max_height = 0;
    let mut visible: Vec<usize> = vec![];
    for (i, height) in row.iter().enumerate() {
        if i == 0 || height > &max_height {
            visible.push(i);
            max_height = *height;
        }
    }
    let mut max_height = 0;
    for (i, height) in row.iter().rev().enumerate() {
        if i == 0 || height > &max_height {
            visible.push(row.len() - i - 1);
            max_height = *height;
        }
    }
    visible
}

fn check_scenic_scores(row: &Vec<i32>) -> Vec<usize> {
    let mut scenic: Vec<usize> = vec![0].repeat(row.len());
    println!("{:?}", row);
    println!("{:?}", row.len());
    for (i, height) in row.iter().enumerate() {
        println!("checking for index {:?} with height {:?}", i, height);
        for (j, next_height) in row[(i + 1)..].iter().enumerate() {
            println!(
                "   against index {:?} with height {:?}",
                i + j + 1,
                next_height
            );
            if height <= next_height {
                scenic[i] = j + 1;
                println!("   scenic score of {:?}", j + 1);
                break;
            }
            if (i + j + 2) == row.len() {
                scenic[i] = j + 1;
                println!("   scenic (end) score of {:?}", j + 1);
            }
        }
    }
    scenic
}
