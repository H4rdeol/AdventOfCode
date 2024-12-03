use std::fs::File;
use std::io::Read;
use regex::Regex;

fn get_file() -> Vec<String> {
    let mut f = File::open("assets/input.txt").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let array: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    array
}

fn get_active_expression() -> Vec<String> {
    let mut f = File::open("assets/input.txt").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let array: Vec<String> = contents.split("don't()").map(|s| s.to_string()).collect();
    let mut new_array: Vec<String> = vec![array[0].to_string()];

    for i in 1..array.len() {
        match array[i].find("do()") {
            Some(a) => {
                new_array.push(array[i][a..].to_string());
            },
            None => {}
        }
    }
    new_array
}

fn calculate_mul(array: Vec<String>) -> i32 {
    let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)");
    let mut res = 0;

    for line in array.iter() {
        for cap in mul_regex.as_ref().expect("No found").captures_iter(line) {
            let left_number = cap[0].split("(").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            let right_number = cap[0].split("(").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[1].split(")").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            res += left_number * right_number;
        }
    }
    res
}

fn main() {
    let array: Vec<String> = get_file();
    println!("step-1: {:?}", calculate_mul(array));
    let array: Vec<String> = get_active_expression();
    println!("step-2: {:?}", calculate_mul(array));
}
