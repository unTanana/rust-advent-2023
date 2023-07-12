// _ _ _ _ _  _  _  _ _  _
// 6 8 2 7 44 2  33 7 11 14

//z0/0/0/6
//y0/0/6/8
//x0/6/8/44

pub fn day1() {
    let file = std::fs::read_to_string("calories").expect("file must exist");
    let mut max1: usize = 0;
    let mut max2: usize = 0;
    let mut max3: usize = 0;

    let mut current: usize = 0;
    file.lines().for_each(|line| {
        if !line.is_empty() {
            current += line.parse::<usize>().expect("must parse");
        } else {
            if current > max1 {
                max3 = max2;
                max2 = max1;
                max1 = current;
            } else if current > max2 {
                max3 = max2;
                max2 = current;
            } else if current > max3 {
                max3 = current;
            }
            current = 0;
        }
    });

    if current > max1 {
        max3 = max2;
        max2 = max1;
        max1 = current;
    } else if current > max2 {
        max3 = max2;
        max2 = current;
    } else if current > max3 {
        max3 = current;
    }

    println!("{}", max1);
    println!("{}", max2);
    println!("{}", max3);
    println!("{}", max1 + max2 + max3);
}
