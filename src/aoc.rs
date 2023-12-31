pub mod day_one;
pub mod day_two;
pub mod day_three;
pub mod day_four;

use std::{fs::File, path::Path};
use std::io::Read;
use std::collections::HashMap;

pub struct Config<'a> {
    day: i32,
    problem: i32,
    path: &'a Path,
}

impl<'a> Config<'a>{
    pub fn new(day: i32, problem: i32, path: &String) -> Config{
        Config {
            day,
            problem,
            path: Path::new(path),
        }
    }
}

pub fn get_file_content(path: &Path) -> String {
    let mut file = File::open(path).expect("The file could not be opened");
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => return contents,
        Err(_) => {
            eprintln!("error encountered opening the file, returning empty string");
            return String::new();
        }
    };

}

pub fn check_replace(s: &mut String, map: &HashMap<&str, char>) -> () {
    let mut buffer = String::new();
    let mut done_flag = false;
    for c in s.clone().chars().into_iter() {
        if done_flag {break;};
        if buffer.len() > 3 {
            for value in map.clone() {
                let (key, num) = value;
                if buffer.contains(key) {
                    *s = s.replacen(key, &num.to_string(), 1);
                    buffer.clear();
                    done_flag = true;
                    break;
                };
            }
        }
        buffer.push(c);
    }

    done_flag = false;
    for c in s.clone().chars().into_iter().rev() {
        if done_flag {break;};
        if buffer.len() > 3 {
            for value in map.clone() {
                let (key, num) = value;
                if buffer.contains(key) {
                    *s = s.replacen(key, &num.to_string(), 1);
                    buffer.clear();
                    done_flag = true;
                    break;
                };
            }
        }
        buffer.insert(0, c);
    }

}

pub fn convert_number_strings(s: &mut String) -> () {
    let map = HashMap::from([
                            ("one", '1'),
                            ("two", '2'),
                            ("three", '3'),
                            ("four", '4'),
                            ("five", '5'),
                            ("six", '6'),
                            ("seven", '7'),
                            ("eight", '8'),
                            ("nine", '9'),
    ]);
    let str_vec = s.split("\n").collect::<Vec<&str>>();
    let replaced_str_vec = str_vec.into_iter().map(|x|{
        let mut x_string = x.to_string();
        check_replace(&mut x_string, &map);
        x_string
    }).collect::<Vec<String>>();
    *s = replaced_str_vec.join("\n");
}

pub fn get_numbers(s: &String) -> Vec<String> {
    let s_vector: Vec<&str> = s.split("\n").collect();
    let mut nums: Vec<String> = Vec::new();
    for item in s_vector.iter(){
        let mut line_nums = String::from("");
        for c in item.chars() {
            if c.is_numeric() {
                line_nums.push(c);
            }
        }
        if !line_nums.is_empty() {
            let mut clean_line = String::with_capacity(2);
            if line_nums.len() == 1 {
                clean_line.insert(0, line_nums.as_bytes()[0] as char);
                clean_line.insert(1, line_nums.as_bytes()[0] as char);
            } else {
                clean_line.insert(0, line_nums.as_bytes()[0] as char);
                clean_line.insert(1, line_nums.as_bytes()[line_nums.len() - 1] as char);
            }
            nums.push(clean_line)
        };
    }
    return nums;
}

pub fn add_string_numbers(s: &Vec<String>) -> usize {
    let sum_values: usize = s.into_iter().map(|x| x.parse::<usize>().expect("The value {} is not a number")).sum();
    return sum_values;
}

pub fn run(instructions: Config){
    match instructions {
        Config {day: 1, problem: 1, ..} => crate::aoc::day_one::first_problem(instructions.path),
        Config {day: 1, problem: 2, ..} => crate::aoc::day_one::second_problem(instructions.path),
        Config {day: 2, problem: 1, ..} => crate::aoc::day_two::first_problem(instructions.path),
        Config {day: 2, problem: 2, ..} => crate::aoc::day_two::second_problem(instructions.path),
        Config {day: 3, problem: 1, ..} => crate::aoc::day_three::first_problem(instructions.path),
        Config {day: 3, problem: 2, ..} => crate::aoc::day_three::second_problem(instructions.path),
        Config {day: 4, problem: 1, ..} => crate::aoc::day_four::first_problem(instructions.path),
        Config {day: 4, problem: 2, ..} => crate::aoc::day_four::second_problem(instructions.path),
        _ => println!("Either this day wasn't completed, or doesn't exist sorry"),
    }
}
