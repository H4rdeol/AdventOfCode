use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut f = File::open("assets/input.txt")?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    
    let array: Vec<&str> = contents.split("\n").collect();
    let mut array_col_1: Vec<i32> = array.iter()
        .filter_map(|x| x.split(" ").next()?.parse::<i32>().ok())
        .collect();
    let mut array_col_2: Vec<i32> = array.iter()
        .filter_map(|x| x.split("   ").nth(1)?.parse::<i32>().ok())
        .collect();

    array_col_1.sort();
    array_col_2.sort();
    
    let result: Vec<i32> = array_col_1.iter().zip(array_col_2.iter())
        .map(|(x, y)| (x - y).abs())
        .collect();
    let mut result2: i32 = 0;
    
    for x in array_col_1.iter() {
        result2 += x * array_col_2.iter().filter(|a| x == *a).count() as i32;
    }
    println!("step-1: {:?}", result.iter().sum::<i32>());
    println!("step-2: {:?}", result2);
    Ok(())
}
