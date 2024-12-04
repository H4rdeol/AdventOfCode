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

fn get_matrix3x3(array: &Vec<String>, i: i32, j: i32) -> Option<Vec<String>> {
    let mut matrix: Vec<String> = Vec::new();

    for x in -1..2 {
        if i + x >= 0 && j - 1 >= 0 && i + x < array.len() as i32 && j + 1 < array[0].len() as i32 {
            matrix.push(array[(i + x) as usize][(j - 1) as usize..(j + 2) as usize].to_string());
        } else {
            return None;
        }
    }
    Some(matrix)
}

fn step_1(array: &Vec<String>) -> i32 {
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
    count
}

fn step_2(array: &Vec<String>) -> i32 {
    let mut res = 0;

    for i in 0..(array.len() as i32) {
        for j in 0..(array[i as usize].len() as i32) {
            if array[i as usize].chars().nth(j as usize).unwrap() == 'A' {
                if let Some(a) =  get_matrix3x3(array, i, j) {
                    let count = line_find(&a, 0, 0, (1, 1), "MAS") +
                                line_find(&a, 2, 0, (-1, 1), "MAS") +
                                line_find(&a, 0, 2, (1, -1), "MAS") + 
                                line_find(&a, 2, 2, (-1, -1), "MAS");
                    if count >= 2 {
                        res += 1;
                    }
                } 
            }
        }
    }
    res
}

fn main() {
    let array: Vec<String> = get_file();
    
    println!("step-1: {:?}", step_1(&array));
    println!("step-2: {:?}", step_2(&array));
}
