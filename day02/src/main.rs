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

fn is_sorted(array: Vec<i32>) -> bool {
    let mut ascending = true;
    let mut descending = true;
    for i in 1..array.len() {
        if array[i - 1] > array[i] || (array[i - 1] - array[i]).abs() == 0 || (array[i - 1] - array[i]).abs() > 3 {
            ascending = false;
        }
    }
    for i in 1..array.len() {
        if array[i - 1] < array[i] || 0 == (array[i - 1] - array[i]).abs() || (array[i - 1] - array[i]).abs() > 3 {
            descending = false;
        }
    }
    ascending || descending
}

fn main() {
    let array: Vec<String> = get_file();
    let mut count1 = 0;
    let mut count2 = 0;

    for x in array.iter() {
        let line_int: Vec<i32> = x.split_whitespace().collect::<Vec<&str>>().iter()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        if is_sorted(line_int.clone()) {
            count1 += 1;
        } else {
            for i in 0..line_int.len() {
                let mut temp = line_int.clone();
                temp.remove(i);
                if is_sorted(temp.clone()) {
                    count2 += 1;
                    break;
                }
            }
        }
    }

    println!("step-1: {:?}", count1);
    println!("step-2: {:?}", count2 + count1);
}
