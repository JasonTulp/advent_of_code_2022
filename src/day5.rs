use super::*;

fn parse_crates(stack: Vec<String>) -> Vec<Vec<char>> {
    let stack: Vec<String> = stack.into_iter().rev().collect();
    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut stack_indices: Vec<usize> = vec![];

    for i in 0..9 {
        crates.push(vec![]);
    }

    for (i, c) in stack[0].chars().enumerate() {
        if c != ' ' {
            stack_indices.push(i);
        }
    }

    for s in 1..stack.len() {
        let stack_chars: Vec<char> = stack[s].chars().collect();
        for index in 0..stack_indices.len() {
            let i = stack_indices[index];
            if stack_chars[i].is_alphabetic() {
                crates[index].push(stack_chars[i]);
            }
        }
    }
    crates.clone()
}

fn move_crates(crates: &mut Vec<Vec<char>>, from: usize, to: usize, amount: usize) {
    let from = from - 1;
    let to = to - 1;

    for _ in 0..amount {
        if crates[from].len() > 0 {
            let c = crates[from].pop().unwrap();
            crates[to].push(c);
        }
    }
}

fn move_crates_in_order(crates: &mut Vec<Vec<char>>, from: usize, to: usize, amount: usize) {
    let from = from - 1;
    let to = to - 1;
    let amount = amount;
    let mut moving_crates: Vec<char> = vec![];

    for _ in 0..amount {
        if crates[from].len() > 0 {
            moving_crates.insert(0, crates[from].pop().unwrap());
        }
    }

    for c in moving_crates {
        crates[to].push(c);
    }
}

fn get_result(crates: &Vec<Vec<char>>) -> String {
    let mut result: String = String::new();
    println!("crates: {:?}", crates);
    for c in crates {
        if (c.len() > 0) {
            let top = c[c.len() - 1];
            result.push(top);
        }
    }
    result
}

pub(crate) fn supply_stacks() {
    let path = "src/inputs/day5.txt";
    let mut stacks: Vec<String> = vec![];
    let mut crates: Vec<Vec<char>> = vec![];
    // First part
    for (i, line) in read_file(path).into_iter().enumerate() {
        if let Ok(line) = line {
            if i < 9 {
                stacks.push(line);
                continue;
            }
            if i == 9 {
                // we have all the stacks, lets parse the input
                crates = parse_crates(stacks.clone());
                println!("Crates: {:?}", crates);
                continue;
            }
            if i > 9 {
                let digest: Vec<String> = line.split(' ').map(|s| String::from(s)).collect();
                let from: usize = digest[3].parse().unwrap();
                let to: usize = digest[5].parse().unwrap();
                let count: usize = digest[1].parse().unwrap();
                move_crates_in_order(&mut crates, from, to, count);
            }
        }
    };

    println!("Result: {:?}", get_result(&crates) );
    // Part 1: ZSQVCCJLL
    // Part 2: QZFJRWHGS
}
