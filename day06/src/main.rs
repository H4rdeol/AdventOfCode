use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
struct Guard {
    x: i32,
    y: i32,
    direction: (i32, i32),
}

impl Guard {
    fn new(x: i32, y: i32, direction: &str) -> Guard {
        Guard {
            x: x,
            y: y,
            direction: match direction {
                ">" => (1, 0),
                "<" => (-1, 0),
                "^" => (0, -1),
                "v" => (0, 1),
                _ => (0, 0)
            },
        }
    }

    fn new_with_direction(x: i32, y: i32, direction: (i32, i32)) -> Guard {
        Guard {
            x: x,
            y: y,
            direction: direction,
        }
    }

    fn rotate(&mut self) {
        let (x, y) = self.direction;
        self.direction = match (x, y) {
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => (0, 0),
        };
    }
}

impl Clone for Guard {
    fn clone(&self) -> Guard {
        Guard::new_with_direction(self.x, self.y, self.direction)
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

fn find_guard(map: &Vec<String>) -> Option<Guard> {
    let mut i: usize = 0;
    let mut j: usize = 0;
    const GUARDS: &str = "><^v";

    while i < map.len() {
        if j >= map[i].len() {
            j = 0;
            i += 1;
        }
        if GUARDS.contains(&map[i][j..j + 1]) {
            return Some(Guard::new(j as i32, i as i32, &map[i][j..j + 1]));
        }
        j += 1;
    }
    None
}

fn move_guard(mut guard: Guard, mut map: Vec<String>, steps: &mut Vec<Guard>) -> bool {
    map[guard.y as usize].replace_range(guard.x as usize..guard.x as usize + 1, ".");
    loop {
        let x = guard.x + guard.direction.0;
        let y = guard.y + guard.direction.1;
        if x < 0 || y < 0 || x >= map[0].len() as i32 || y >= map.len() as i32 {
            break;
        }
        if map[y as usize].chars().nth(x as usize).unwrap() == '#' {
            guard.rotate();
            continue;
        }
        if steps.contains(&guard) {
            return false;
        }
        if map[guard.y as usize].chars().nth(guard.x as usize).unwrap() == ".".chars().nth(0).unwrap() {
            map[guard.y as usize].replace_range(guard.x as usize..guard.x as usize + 1, "X");
            steps.push(guard.clone());
        }
        guard.x = x;
        guard.y = y;
    }
    steps.push(Guard::new_with_direction(guard.x, guard.y, guard.direction));
    true
}

fn find_infinit_loop(steps: Vec<Guard>, map: Vec<String>, guard: &Guard) {
    let mut nb_loop: usize = 0;
    let mut already_obstruct: Vec<(i32, i32)> = Vec::new();
    
    for step in steps.iter() {
        if already_obstruct.contains(&(step.x, step.y)) {
            continue;
        }
        let mut new_map = map.clone();
        let mut new_steps: Vec<Guard> = Vec::new();
        new_map[step.y as usize].replace_range(step.x as usize..step.x as usize + 1, "#");
        already_obstruct.push((step.x, step.y));
        if !move_guard(guard.clone(), new_map, &mut new_steps) {
            nb_loop += 1;
        }
    }
    println!("step-2: {:?}", nb_loop);
}

fn main() {
    let map = get_file();
    let guard = match find_guard(&map) {
        Some(guard) => guard,
        None => {
            println!("No guard found");
            return;
        }
    };
    let mut steps: Vec<Guard> = Vec::new();

    move_guard(guard.clone(), map.clone(), &mut steps);
    println!("step-1: {:?}", steps.len());
    find_infinit_loop(steps, map, &guard);
}
