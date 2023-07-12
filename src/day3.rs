pub fn get_total_priority(input: &str) -> usize {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                (c as usize) - 96
            } else {
                (c as usize) - 38
            }
        })
        .sum()
}

pub fn get_common_characters(a: &str, b: &str) -> String {
    let mut holder = [0; 123];

    a.chars().for_each(|c| {
        let existing = holder[c as usize];
        if existing == 0 {
            holder[c as usize] = 1;
        }
    });

    b.chars().for_each(|c| {
        let existing = holder[c as usize];
        if existing == 1 {
            holder[c as usize] = 2;
        }
    });

    holder
        .iter()
        .enumerate()
        .filter(|(_, num)| **num == 2)
        .map(|(idx, _)| idx as u8 as char)
        .collect::<String>()
}

pub fn get_common_characters_3(a: &str, b: &str, c: &str) -> String {
    let mut holder = [0; 123];

    a.chars().for_each(|chr| {
        let existing = holder[chr as usize];
        if existing == 0 {
            holder[chr as usize] = 1;
        }
    });
    b.chars().for_each(|chr| {
        let existing = holder[chr as usize];
        if existing == 1 {
            holder[chr as usize] = 2;
        }
    });
    c.chars().for_each(|chr| {
        let existing = holder[chr as usize];
        if existing == 2 {
            holder[chr as usize] = 3;
        }
    });

    holder
        .iter()
        .enumerate()
        .filter(|(_, num)| **num == 3)
        .map(|(idx, _)| idx as u8 as char)
        .collect::<String>()
}

pub fn day3() {
    let file = std::fs::read_to_string("rucksack").expect("File must exist");
    let sum: usize = file
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_at(line.len() / 2);
            let common = get_common_characters(p1, p2);
            get_total_priority(common.as_str())
        })
        .sum();

    println!("{:?}", sum);

    let mut lines_it = file.lines();
    let mut sum2 = 0;
    while let Some(x) = lines_it.next() {
        let y = lines_it.next().expect("must exist");
        let z = lines_it.next().expect("must exist");
        let common = get_common_characters_3(x, y, z);
        sum2 += get_total_priority(common.as_str());
    }

    println!("{:?}", sum2);
}
