use std::fs::File;
use std::io::Read;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Line {
    expected_result: u64,
    numbers: Vec<u64>,
    operation: String,
}

impl Line {
    fn parse_new(line: &str) -> Line {
        let parts: Vec<&str> = line.split(":").collect();
        let expected_result = parts[0].parse::<u64>().unwrap();
        let numbers: Vec<u64> = parts[1].split(" ").skip(1).map(|s| s.parse::<u64>().unwrap()).collect();
        let mut operations = "".to_string();

        for _ in 0..numbers.len() - 1 {
            operations.push_str("+");
        }
        Line::new(expected_result, numbers, operations)
    }

    fn new(expected_result: u64, numbers: Vec<u64>, operation: String) -> Line {
        Line {
            expected_result: expected_result,
            numbers: numbers,
            operation: operation
        }
    }

    fn verify_result(&self) -> bool {
        let mut res = self.numbers[0];
        let operation: Vec<char> = self.operation.chars().collect();
        let numbers = self.numbers.clone();
        
        for j in 1..self.numbers.len() {
            match operation[j - 1] {
                '+' => {
                    res += numbers[j];
                },
                '*' => {
                    res *= numbers[j];
                },
                '|' => {
                    res = concat_numbers(res, numbers[j]);
                },
                char => panic!("Invalid operation {}", char),
            }
        }
        self.expected_result == res
    }
}

fn get_file() -> Vec<String> {
    let mut f = File::open("assets/input.txt").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let array: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    array
}

fn get_lines(array: Vec<String>) -> Vec<Line> {    
    let mut lines: Vec<Line> = Vec::new();

    for i in array.iter() {
        lines.push(Line::parse_new(i));
    }
    lines
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut n = a;
    let mut c = b;

    while c > 0 {
        n *= 10;
        c /= 10;
    }
    n + b
}

fn step1(lines: &mut Vec<Line>) {
    let mut result = 0;
    let operators = vec!['+', '*'];

    for line in lines.iter_mut() {
        let operator_vec = vec![operators.clone(); line.numbers.len() - 1];
        let combinations = operator_vec.iter().multi_cartesian_product();
        for combination in combinations.clone() {
            line.operation = combination.iter().copied().collect::<String>();
            if line.verify_result() {
                result += line.expected_result;
                break;
            }
        }
    }
    println!("step-1: {}", result);
}

fn step2(lines: &mut Vec<Line>) {
    let mut result = 0;
    let operators = vec!['+', '*', '|'];

    for line in lines.iter_mut() {
        let operator_vec = vec![operators.clone(); line.numbers.len() - 1];
        let combinations = operator_vec.iter().multi_cartesian_product();
        for combination in combinations.clone() {
            line.operation = combination.iter().copied().collect::<String>();
            if line.verify_result() {
                result += line.expected_result;
                break;
            }
        }
    }
    println!("step-2: {}", result);
}

fn main() {
    let file = get_file();
    let mut lines = get_lines(file);

    step1(&mut lines);
    step2(&mut lines);
}
