use super::*;

/// Converts a character to a priority
fn char_to_priority(char: char) -> u32 {
    // Lowercase 97 - 122
    // Uppercase 65 - 90
    let ascii = char as u32;
    if ascii == 48 {
        println!("YOU FUCKED UP");
        return 0
    }
    if ascii > 97 {
        return ascii - 96
    }
    ascii - 38
}

fn find_shared_character(left: &str, right: &str) -> char {
    let mut shared = '0';
    left.chars().for_each(|l_char| {
        right.chars().for_each(|r_char| {
            if l_char == r_char {
                shared = l_char;
            }
        })
    });
    shared
}

fn find_identity_badge(group: Vec<String>) -> char {
    let mut shared: Vec<char> = vec![];
    group[0].chars().for_each(|a| {
        group[1].chars().for_each(|b| {
            if a == b {
                shared.push(a);
            }
        })
    });

    let mut badge: char = '0';
    group[2].chars().for_each(|a| {
        shared.iter().for_each(|b| {
            if a == *b {
                badge = a;
            }
        })
    });

    badge
}

pub(crate) fn rucksack_organisation() {
    let path = "src/inputs/day3.txt";
    let mut sum: u32 = 0;

    // First part
    read_file(path).for_each(|line| {
        if let Ok(line) = line {
            let (left, right) = line.split_at(line.len()/ 2);
            let character: char = find_shared_character(left, right);
            sum += char_to_priority(character);
        }
    });

    // Second part
    let mut group: Vec<String> = vec![];
    let mut identity_sum: u32 = 0;
    read_file(path).for_each(|line| {
        if let Ok(line) = line {
            group.push(line);

            if group.len() == 3 {
                let character: char = find_identity_badge(group.clone());
                identity_sum += char_to_priority(character);

                group = vec![];
            }
        }
    });


    println!("Total Sum: {:?}", sum );
    println!("Total Badge Sum: {:?}", identity_sum );
    // Part 1: 8298
    // Part 2: 2708
}