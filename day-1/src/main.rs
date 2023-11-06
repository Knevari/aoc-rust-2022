use std::fs;


fn main() {
    let contents = fs::read_to_string("./elfs-calories.txt").expect("Should have been able to read the file");
    let parts = contents.trim().lines();

    let mut current_count: u32 = 0;
    let mut max_calories: u32 = 0; 

    for part in parts {
       let num: u32 = match part.parse::<u32>() {
           Ok (num) => num,
           Err(_) => {
                current_count = 0;
                continue
           }
       };

       current_count += num;

       if current_count > max_calories {
           max_calories = current_count;
       }
    }

    println!("{max_calories}");
}
