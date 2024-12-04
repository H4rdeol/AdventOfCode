use std::fs::File;
use std::io::Read;

fn get_file() -> Vec<String> {
    let mut f = File::open("assets/input.txt").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let array: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    array
}

fn line_find(array: &Vec<String>, mut i: i32, mut j: i32, direction: (i32, i32), word: &str) -> i32 {
    let mut index = 0;

    while i >= 0 && j >= 0 && i < array.len() as i32 && j < array[0].len() as i32 {
        if index == word.len() {
            break;
        }
        if array[i as usize].chars().nth(j as usize).unwrap() == word.chars().nth(index as usize).unwrap() {
            index += 1;
        } else {
            return 0;
        }
        i += direction.0;
        j += direction.1;
    }
    return if index == word.len() {1} else {0}
}

fn main() {
    let array: Vec<String> = get_file();
    let mut count = 0;

    for i in 0..(array.len() as i32) {
        for j in 0..(array[i as usize].len() as i32) {
            count += line_find(&array, i, j, (0, 1), "XMAS") +
            line_find(&array, i, j, (1, 0), "XMAS") + 
            line_find(&array, i, j, (-1, 0), "XMAS") +
            line_find(&array, i, j, (0, -1), "XMAS") +
            line_find(&array, i, j, (1, 1), "XMAS") +
            line_find(&array, i, j, (-1, -1), "XMAS") +
            line_find(&array, i, j, (-1, 1), "XMAS") +
            line_find(&array, i, j, (1, -1), "XMAS");
        }
    }
    println!("step-1: {:?}", count);
}
