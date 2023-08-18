use super::*;

pub(crate) fn calculate_calories() {
    println!("===== Initiating caloric process");
    let path = "src/inputs/day1.txt";

    let mut elf_calories: Vec<u64> = vec![];
    let mut current_calories: u64 = 0;

    read_file(path).for_each(|line| {
        if let Ok(line) = line {
            if line == "" {
                elf_calories.push(current_calories);
                current_calories = 0;
            } else {
                let calories: u64 = line.parse::<u64>().unwrap_or_default();
                current_calories += calories;
            }
        }
    });

    elf_calories.sort_by(|a, b| b.cmp(a));
    let top_three: u64 = elf_calories.iter().take(3).sum();
    println!("Top three calories: {:?}", top_three);
    // Result: 207410
}