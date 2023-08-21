use super::*;


pub(crate) fn find_packet_marker() {
    let path = "src/inputs/day6.txt";
    let mut prev_chars: Vec<char> = vec![];
    // First part
    let Some((i, line)) = read_file(path).into_iter().enumerate().next()else {
        return;
    };

    if let Ok(line) = line {
        let mut index = 0;
        for c in line.chars() {
            index += 1;

            prev_chars.push(c);
            if prev_chars.len() > 14 {
                prev_chars.remove(0);
            }
            let mut prev_chars_deduped = prev_chars.clone();
            prev_chars_deduped.sort_unstable();
            prev_chars_deduped.dedup();
            if prev_chars.len() == 14 && prev_chars.len() == prev_chars_deduped.len() {
                // We found the packet marker
                println!("Found char index {:?}",index);
                println!("Found a char: {:?}", prev_chars_deduped);

                return;
            }
        }
    }
}
