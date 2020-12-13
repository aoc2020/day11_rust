use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

type Pos = (i64, i64);
type Seat = char;
type SeatMap = HashMap<Pos, Seat>;

lazy_static! {
    static ref DIRS: Vec<Pos> = vec!((1, 1), (1, 0), (1, -1), (0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1));
}

fn is_floor(s: Seat) -> bool {
    s == '.'
}

fn is_taken(s: Seat) -> bool {
    s == '#'
}

fn is_free(s: Seat) -> bool {
    s == 'L'
}

const TAKEN_SEAT: char = '#';
const FREE_SEAT: char = 'L';

fn add(a: &Pos, b: &Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn neighbor_taken(pos: &Pos, dir: &Pos, seats: &SeatMap) -> bool {
    let pos_to_check = add(&pos, &dir);
    seats.contains_key(&pos_to_check) && is_taken(seats[&pos_to_check])
}

fn new_value_task1(pos: &Pos, seats: &SeatMap) -> Seat {
    let seat = seats[pos];
    if is_floor(seat) { return seat; }
    let neighbors = (*DIRS).clone().into_iter()
        .filter(|dir| neighbor_taken(pos, &dir, seats))
        .count();
    if neighbors == 0 { return TAKEN_SEAT; }
    if neighbors > 3 { return FREE_SEAT; }
    seat
}

fn scan_direction(pos: &Pos, dir: &Pos, seats: &SeatMap) -> bool {
    let mut curr = add(pos, dir);
    while seats.contains_key(&curr) {
        if is_taken(seats[&curr]) {
            return true;
        }
        if is_free(seats[&curr]) {
            return false;
        }
        curr = add(&curr, dir);
    }
    false
}

fn new_value_task2(pos: &Pos, seats: &SeatMap) -> Seat {
    if is_floor(seats[pos]) { return seats[pos].clone(); }
    let others = (*DIRS).clone().into_iter()
        .filter(|d| scan_direction(&pos, d, seats))
        .count();
    if others == 0 { return TAKEN_SEAT; }
    if others > 4 { return FREE_SEAT; }
    seats[&pos]
}

fn iterate_map(positions: &Vec<Pos>, seats: &SeatMap, new_value: &fn(&Pos, &SeatMap) -> Seat) -> HashMap<Pos, Seat> {
    let mut new_seats: HashMap<Pos, Seat> = HashMap::new();
    for pos in positions {
        let new_seat = new_value(&pos, seats);
        new_seats.insert(pos.clone(), new_seat);
    }
    new_seats
}

fn eq(map1: &SeatMap, map2: &SeatMap) -> bool {
    for pos in map1.keys() {
        if map1[pos] != map2[pos] {
            return false;
        }
    }
    true
}

fn first_stable(positions: &Vec<Pos>, seats: &SeatMap, new_seat: fn(&Pos, &SeatMap) -> Seat) -> SeatMap {
    let mut map = seats.clone();
    let mut new_map = iterate_map(positions, seats, &new_seat);
    while !eq(&map, &new_map) {
        map = new_map;
        new_map = iterate_map(positions, &map, &new_seat);
    }
    new_map
}

fn occupied(seats: &SeatMap) -> i64 {
    seats.values()
        .filter(|seat| is_taken(seat.clone().clone()))
        .count() as i64
}

fn main() {
    let lines: Vec<String> = read_file("input.txt");
    let seat_map = to_map(&lines);
    let positions: Vec<Pos> = seat_map.keys().map(|k| k.clone()).collect();
    let stable1 = first_stable(&positions, &seat_map, new_value_task1);
    let answer1 = occupied(&stable1);
    println!("Answer 1: {}", answer1);
    let stable2 = first_stable(&positions, &seat_map, new_value_task2);
    let answer2 = occupied(&stable2);
    println!("Answer 2: {}", answer2);
}

fn to_pos_seats(line: &String, y: usize) -> Vec<(Pos, Seat)> {
    line.chars()
        .enumerate()
        .map(|ec| ((ec.0 as i64, y as i64), ec.1))
        .collect()
}

fn print_seats(seats: &SeatMap) {
    let mut y: i64 = 0;
    while seats.contains_key(&(0, y.clone())) {
        let mut x: i64 = 0;
        while seats.contains_key(&(x.clone(), y.clone())) {
            let pos = (x.clone(), y.clone());
            let seat = seats[&pos];
            print!("{}", seat);
            x += 1;
        }
        println!();
        y += 1;
    }
}

fn to_map(lines: &Vec<String>) -> SeatMap {
    let positioned: Vec<(Pos, Seat)> =
        lines.clone().iter()
            .enumerate()
            .flat_map(|el| to_pos_seats(el.1, el.0))
            .collect();
    let mut pos_map: HashMap<Pos, Seat> = HashMap::new();
    for pos_seat in positioned {
        pos_map.insert(pos_seat.0, pos_seat.1);
    }
    pos_map
}


fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect(format!("File not found: {}", filename).as_str());
    let reader = BufReader::new(file);
    reader.lines().map(|s| s.expect("Could not collect line")).collect()
}