fn add_and_offset_to_arr(arr: &mut [char], chr: char) {
    for i in (1..arr.len()).rev() {
        arr[i] = arr[i - 1];
    }

    arr[0] = chr;
}

fn is_array_of_uniques(arr: &[char]) -> bool {
    if arr.contains(&'_') {
        return false;
    }

    for i in 0..arr.len() - 1 {
        for j in i + 1..arr.len() {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }

    true
}

pub fn day6() {
    let file = std::fs::read_to_string("marker").expect("file exists");

    let mut holder = ['_'; 14];

    let mut count = 0;
    file.chars()
        .take_while(|new_char| {
            count += 1;
            add_and_offset_to_arr(&mut holder, *new_char);
            let is_unique = is_array_of_uniques(&holder);

            println!("{:?} -> {}", holder, is_unique);

            !is_unique
        })
        .count();

    println!("{:?}", count);
}
