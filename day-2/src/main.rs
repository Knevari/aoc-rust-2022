use std::fs;

fn main() {
    let mut top_1: u32 = 0;
    let mut top_2: u32 = 0;
    let mut top_3: u32 = 0;

    let contents = fs::read_to_string("./elfs-calories.txt").expect("Should be able to read file");

    let parts = contents.trim().lines();

    let mut count: u32 = 0;

    for part in parts {
        let num: u32 = match part.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                count = 0;
                continue
            }
        };

        count += num;

        if count > top_1 {
            top_3 = top_2;
            top_2 = top_1;
            top_1 = count;
        }
    }

    let total = top_1 + top_2 + top_3;

    println!("{total}");
}
