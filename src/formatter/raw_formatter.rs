use std::{collections::HashMap, fs};

use chrono::NaiveDateTime;

use crate::input_file::InputFile;

use super::Formatter;

pub struct RawFormatter<'a> {
    data: &'a Vec<InputFile>,
}

impl<'a> RawFormatter<'a> {
    pub fn new(data: &'a Vec<InputFile>) -> Self {
        return Self { data };
    }
}

impl<'a> Formatter for RawFormatter<'a> {
    fn compile(&self) {
        let mut header = "Date".to_owned();
        let mut device_found: Vec<&String> = Vec::new();
        // Get all devices ID concerned
        for file in self.data {
            if !device_found.contains(&file.device()) {
                header = format!("{};{}", header, file.device());
                device_found.push(file.device());
            }
        }
        // Insert in big group of events no matter the sorting nor the empty fields
        let mut list: Vec<String> = Vec::new();
        for file in self.data {
            for evt in file.events() {
                let mut data = format!("{}", evt.date());
                for id in &device_found {
                    if id == &file.device() {
                        data = format!("{};{}", data, evt.battery());
                    } else {
                        data = format!("{};", data);
                    }
                }
                list.push(data);
            }
        }
        // Sort and add previous data
        list.sort_by(|a, b| {
            let a_str = a.split(";").next().unwrap();
            let b_str = b.split(";").next().unwrap();
            let date_time_a = match NaiveDateTime::parse_from_str(&a_str, "%d-%m-%Y %H:%M:%S") {
                Ok(d) => d,
                Err(e) => {
                    println!("Error : {} on {}", e.to_owned(), a_str);
                    panic!();
                }
            };
            let date_time_b = match NaiveDateTime::parse_from_str(&b_str, "%d-%m-%Y %H:%M:%S") {
                Ok(d) => d,
                Err(e) => {
                    println!("Error : {} on {}", e.to_owned(), b_str);
                    panic!();
                }
            };
            return date_time_a.cmp(&date_time_b);
        });
        let mut prev_data = HashMap::new();
        for (idx, _) in device_found.iter().enumerate() {
            prev_data.insert(idx, "0".to_string());
        }
        let mut complete_data = Vec::new();
        for evt in list {
            let mut joined_data = format!("");

            let mut evt_splitted = evt.split(";").collect::<Vec<&str>>();
            for (idx, evt) in evt_splitted.iter_mut().enumerate() {
                if idx == 0 {
                    joined_data = format!("{}", evt);
                } else if evt == &"" {
                    joined_data = format!("{};{}", joined_data, prev_data[&(idx - 1)]);
                } else {
                    joined_data = format!("{};{}", joined_data, evt);
                    let prev = prev_data.get_mut(&(idx - 1)).unwrap();
                    *prev = evt.to_string();
                }
            }
            complete_data.push(joined_data);
        }
        complete_data.insert(0, header);
        // Write in the file
        fs::write("raw_output.csv", complete_data.join("\n")).expect("");
    }
}
