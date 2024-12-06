use std::fmt::Error;
use std::fs::File;
use std::io::Read;
use std::slice::Iter;

fn get_file() -> Vec<String> {
    let mut f = File::open("assets/input.txt").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let array: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    array
}

fn create_first_tab(array: &mut Iter<'_, String>) -> Result<Vec<(i32, i32)>, Error> {
    let mut orders: Vec<(i32, i32)> = Vec::new();

    while let Some(a) = array.next() {
        let str = a.to_string();
        if str.len() == 0 {
            break;
        }
        match (str.split('|').collect::<Vec<&str>>()[0].parse::<i32>(), str.split('|').collect::<Vec<&str>>()[1].parse::<i32>()) {
            (Ok(a), Ok(b)) => orders.push((a, b)),
            (Err(_), _) => return Err(Error),
            (_, Err(_)) => return Err(Error)
        }
    }
    Ok(orders)
}

fn verify_rule(line_int: &Vec<i32>, orders: &Vec<(i32, i32)>) -> bool {

    for i in 1..line_int.len() {
        let rules: Vec<i32> = orders.iter().filter(|(a, _)| a == line_int.get(i).unwrap()).map(|(_, b)| *b).collect();
        for j in 0..i {
            if rules.contains(line_int.get(j).unwrap()) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let array = get_file();
    let mut iter = array.iter();
    let mut page_numbers_step1: Vec<i32> = Vec::new();
    
    if let Ok(orders) = create_first_tab(&mut iter) {
        while let Some(line) = iter.next() {
            let line_int: Vec<i32> = line.split(',').collect::<Vec<&str>>().iter()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
            if verify_rule(&line_int, &orders) {
                page_numbers_step1.push(*line_int.get(line_int.len() / 2).unwrap());
            }
        }
        println!("step-1: {:?}", page_numbers_step1.iter().sum::<i32>());
    } else {
        println!("Error");
    }
}
