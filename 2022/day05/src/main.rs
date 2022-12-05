use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should be able to read file");
    let num_stacks = (contents.lines().collect::<Vec<&str>>()[0].len() + 4 - 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for stack in 0..num_stacks {
        stacks.push(Vec::new());
    }
    let mut parsing_stacks: bool = true;
    'line: for line in contents.lines() {
        println!("{}", line);
        'stack: for stack in 0..num_stacks {
            if !parsing_stacks {
                continue
            }
            let stack_char = line.chars().nth(stack*4 + 1).unwrap();
            if stack_char == ' ' {
                continue;
            }
            if stack_char == '1' {
                for stack in 0..num_stacks {
                    stacks[stack].reverse();
                }
                parsing_stacks = false;
                break 'stack;
            }
            stacks[stack].push(stack_char);
        }
        let line_split = line.split(' ').collect::<Vec<_>>();
        if parsing_stacks || line_split.len() != 6 {
            continue
        }
        let mut tmp_stack = Vec::new();
        let num_move: usize = line_split[1].parse().unwrap();
        let from_stack_idx: i32 = line_split[3].parse::<i32>().unwrap() - 1;
        let to_stack_idx: i32 = line_split[5].parse::<i32>().unwrap() - 1;
        let from_stack_len = stacks[from_stack_idx as usize].len();
        tmp_stack.extend(stacks[from_stack_idx as usize].drain(from_stack_len-num_move..from_stack_len));
        // tmp_stack.reverse();
        stacks[to_stack_idx as usize].extend(tmp_stack);

    }

    let solution: String = stacks.iter().filter_map(|s| s.last()).collect();
    println!("{}", solution)
}
