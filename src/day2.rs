use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn _move(&mut self, direction: &str, amount: i32) {
        match direction {
            "forward" => self.x += amount,
            "up" => self.y -= amount,
            "down" => self.y += amount,
            _ => (),
        }
    }
}

pub fn day2_p1() -> i32 {
    let file = File::open("inputs/day2_p1.txt").expect("File not found");
    let commands = BufReader::new(file).lines();

    let mut position: Position = Position { x: 0, y: 0 };
    for command in commands.flatten() {
        let v: Vec<&str> = command.split(' ').collect();
        position._move(v[0], v[1].parse::<i32>().unwrap());
    }

    return position.x * position.y;
}

struct Position2 {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position2 {
    fn _move(&mut self, direction: &str, amount: i32) {
        match direction {
            "forward" => {
                self.x += amount;
                self.y += self.aim * amount
            }
            "up" => self.aim -= amount,
            "down" => self.aim += amount,
            _ => (),
        }
    }
}

pub fn day2_p2() -> i32 {
    let file = File::open("inputs/day2_p1.txt").expect("File not found");
    let commands = BufReader::new(file).lines();

    let mut position: Position2 = Position2 { x: 0, y: 0, aim: 0 };
    for command in commands.flatten() {
        let v: Vec<&str> = command.split(' ').collect();
        position._move(v[0], v[1].parse::<i32>().unwrap());
    }

    return position.x * position.y;
}
