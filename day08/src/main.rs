use std::fs::File;
use std::io::Read;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

fn get_file() -> Vec<String> {
    let mut f = File::open("assets/input.txt").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let array: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    array
}

fn init_hashmap(map: &Vec<String>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas = HashMap::new();
    for i in 0..map.len() {
        for j in map[i].chars().enumerate() {
            if j.1 != '.' {
                if !antennas.contains_key(&j.1) {
                    antennas.insert(j.1, vec![(i as i32, j.0 as i32)]);
                } else {
                    antennas.get_mut(&j.1).unwrap().push((i as i32, j.0 as i32));
                }
            }
        }
    }
    antennas
}

fn main() {
    let map = get_file();
    let antennas = init_hashmap(&map);
    let mut result_step1: HashSet<(i32, i32)> = HashSet::new();
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for (_, value) in antennas.iter() {
        let pairs: Vec<_> = iproduct!(
            value, 
            value
        ).collect();
        
        for (c1, c2) in pairs {
            if c1 == c2 {
                continue;
            }
            let distance = (c2.0 - c1.0, c2.1 - c1.1);
            let inv_distance = (-distance.0, -distance.1);
            let coord1 = (c1.0 + inv_distance.0, c1.1 + inv_distance.1);
            let mut coord2 = (c1.0 + distance.0, c1.1 + distance.1);
            if coord1.0 >= 0 && coord1.1 >= 0 && coord1.0 < (map.len() - 1) as i32 && coord1.1 < map[0].len() as i32 {
                result_step1.insert(coord1);
            }
            while !(coord2.0 < 0 || coord2.1 < 0 || coord2.0 >= (map.len() - 1) as i32 || coord2.1 >= map[0].len() as i32) {
                result.insert(coord2);
                coord2 = (coord2.0 + distance.0, coord2.1 + distance.1);
            }
        }
    }
    println!("step-1: {}", result_step1.len());
    println!("step-2: {}", result.len());
}
