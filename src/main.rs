use std::{fs, vec};
use regex::Regex;

const FILE_PATH: &str = "./data/1.txt";

fn main() {
    let data = get_data_from_file();
    let time = get_time_vector(data);
    calculate_time(time);
}

fn get_data_from_file() -> String {
    let result = fs::read_to_string(String::from(FILE_PATH)).unwrap();

    result
}

fn get_time_vector(data: String) -> Vec<Vec<f32>> {
    let pattern = r"(\d){0,2}:(\d){0,2}";
    let regex = Regex::new(pattern).unwrap();

    let mut result:Vec<Vec<f32>> = vec![];

    for item in regex.captures_iter(&data) {
        let item_result = item.get(0).unwrap().as_str();
        let time = item_result
            .split(':')
            .map(|item| item.parse::<f32>()
            .unwrap())
            .collect::<Vec<_>>();
        result.push(time);
    }

    result
}

fn calculate_time(time: Vec<Vec<f32>>) {
    let mut total_hours = 0.0;
    let mut total_minutes = 0.0;

    for span in time {
        if let [hours, minutes] = span.as_slice() {
            total_hours += hours;
            total_minutes += minutes;
        };
    }

    total_hours += total_minutes / 60.0;
    total_minutes = total_minutes % 60.0;

    let formated_string = format!("{:02}:{:02}", total_hours.floor(), total_minutes);

    println!("{}", formated_string);
}