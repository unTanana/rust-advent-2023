// 3      7     1     9
// 0      9     1     6
// r1x   r1y   r2x   r2y
// (r1x >= r2x && r1y <= r2y) ||
// (r2x >= r1x && r2y <= r1y)

fn do_sections_full_overlap(r1x: &str, r1y: &str, r2x: &str, r2y: &str) -> bool {
    let r1x: usize = r1x.parse().expect("must parse");
    let r2x: usize = r2x.parse().expect("must parse");
    let r1y: usize = r1y.parse().expect("must parse");
    let r2y: usize = r2y.parse().expect("must parse");

    (r1x >= r2x && r1y <= r2y) || (r2x >= r1x && r2y <= r1y)
}
// -r1x-------------r2x-----r1y-------------r2y---------------------
//  (r1x <= r2x && r1y >= r2x) || (r1y >= r2x && r1y <= r2y)
// -r2x-------------r1x----r2y-------------r1y---------------------
//  (r2x<= r1x && r1x >= r2y) || ( r2y>= r1x && r2y<= r1y)

fn do_sections_overlap(r1x: &str, r1y: &str, r2x: &str, r2y: &str) -> bool {
    let r1x: usize = r1x.parse().expect("must parse");
    let r2x: usize = r2x.parse().expect("must parse");
    let r1y: usize = r1y.parse().expect("must parse");
    let r2y: usize = r2y.parse().expect("must parse");

    r1x <= r2y && r1y >= r2x
}

pub fn day4() {
    let file = std::fs::read_to_string("sections").expect("file must exist");
    let mut full_overlaps = 0;
    let mut overlaps = 0;
    file.lines().for_each(|line| {
        let (p1, p2) = line.split_once(',').expect("must split ,");
        let (r1x, r1y) = p1.split_once('-').expect("must split -");
        let (r2x, r2y) = p2.split_once('-').expect("must split -");

        if do_sections_full_overlap(r1x, r1y, r2x, r2y) {
            full_overlaps += 1;
        }

        if do_sections_overlap(r1x, r1y, r2x, r2y) {
            overlaps += 1;
        }
    });

    println!("{:?}", full_overlaps);
    println!("{:?}", overlaps);
}
