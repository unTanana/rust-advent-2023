// 1 / 4 + 1 = 1
// 5 / 4 + 1 = 2
// 9 / 4 + 1 = 3
// 13 / 4 + 1 = 4

fn get_stack_from_idx(idx: usize) -> usize {
    idx / 4
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_move(mv: &str) -> Result<Move, anyhow::Error> {
    let parts: Vec<&str> = mv.split(' ').collect();
    Ok(Move {
        amount: parts[1].parse()?,
        from: parts[3].parse::<usize>().unwrap() - 1,
        to: parts[5].parse::<usize>().unwrap() - 1,
    })
}

fn make_move(amount: usize, from: usize, to: usize, stacker: &mut Vec<Vec<char>>) {
    for i in 0..amount {
        let from = stacker.get_mut(from).unwrap();
        if let Some(c) = from.pop() {
            stacker.get_mut(to).unwrap().push(c);
        } else {
            panic!("should POP!")
        }
    }
}

pub fn day5() {
    let file = std::fs::read_to_string("crates").expect("file exists");
    let mut stacker: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    file.lines()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            line.chars().enumerate().for_each(|(idx, char)| {
                if char.is_ascii_uppercase() {
                    let stack_number = get_stack_from_idx(idx);
                    stacker[stack_number].insert(0, char);
                }
            })
        });

    println!("{:#?}", stacker);

    let mut is_moving = false;
    file.lines().for_each(|line| {
        if is_moving {
            let mv = parse_move(line).expect("should parse every move");
            make_move(mv.amount, mv.from, mv.to, &mut stacker);
        }

        if line.is_empty() {
            is_moving = true;
        }
    });

    println!("{:#?} \n", stacker);
    stacker.iter().for_each(|stack| {
        println!("{}", stack.last().unwrap_or(&'_'));
    })
}
