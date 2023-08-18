use super::*;

pub(crate) fn camp_cleanup() {
    let path = "src/inputs/day4.txt";
    let mut sum: u32 = 0;

    // First part
    read_file(path).for_each(|line| {
        if let Ok(line) = line {
            let digest: Vec<&str> = line.split(',').collect();
            let assignment_1: Vec<&str> = digest[0].split('-').collect();
            let assignment_2: Vec<&str> = digest[1].split('-').collect();

            let a_1: (u64, u64) = (assignment_1[0].parse().unwrap(), assignment_1[1].parse().unwrap());
            let a_2: (u64, u64) = (assignment_2[0].parse().unwrap(), assignment_2[1].parse().unwrap());

            if check_if_contained(a_1.clone(), a_2.clone()) ||
                check_if_overlap(a_1, a_2){
                sum += 1;
            }
        }
    });

    println!("Total Sum: {:?}", sum );
    // Part 1: 576
    // Part 2: 905
}

fn check_if_contained(a_1: (u64, u64), a_2: (u64, u64)) -> bool {
    if a_1.0 >= a_2.0 && a_1.1 <= a_2.1 {
        return true;
    }
    if a_2.0 >= a_1.0 && a_2.1 <= a_1.1 {
        return true;
    }
    false
}

fn check_if_overlap(a_1: (u64, u64), a_2: (u64, u64)) -> bool {
    // .....1234
    // ........4567
    if a_1.0 <= a_2.0 && a_1.1 >= a_2.0 {
        return true;
    }
    // .......4567
    // ....1234
    if a_1.0 <= a_2.1 && a_1.1 >= a_2.1 {
        return true;
    }
    false
}
