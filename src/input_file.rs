use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{error::ParsingError, event_data::EventData};

#[derive(Clone)]
pub struct InputFile {
    gateway: String,
    device: String,
    date: String,
    events: Vec<EventData>,
}

impl TryFrom<PathBuf> for InputFile {
    type Error = ParsingError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return Err(ParsingError::new("File cannot be opened")),
        };
        let name = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name,
                None => return Err(ParsingError::new("Cannot convert the file name")),
            },
            None => return Err(ParsingError::new("Cannot read file name")),
        };
        let mut name_splitted = name.split("-").collect::<Vec<&str>>();
        let date_str_raw = match name_splitted.pop() {
            Some(d) => {
                let splitted = d.split(".").collect::<Vec<&str>>();
                match splitted.get(0) {
                    Some(d) => d.to_owned(),
                    None => return Err(ParsingError::new("Cannot fetch date from file name")),
                }
            }
            None => return Err(ParsingError::new("Cannot fetch date from file name")),
        };
        let date_str = date_str_raw.replace("_", "-");
        let device = match name_splitted.pop() {
            Some(d) => d,
            None => return Err(ParsingError::new("Cannot fetch device id from file name")),
        };
        let gateway_id = name_splitted.join("-");
        let lines = BufReader::new(file).lines();
        let mut vec_evt = Vec::new();
        for line in lines.flatten().skip(1) {
            let mut evt = EventData::try_from(line)?;
            evt.add_date(&date_str);
            vec_evt.push(evt);
        }
        return Ok(Self {
            gateway: gateway_id,
            events: vec_evt,
            device: device.to_string(),
            date: date_str,
        });
    }
}

impl InputFile {
    pub fn new(gateway_id: &str, device: &str, date: &str) -> Self {
        return Self {
            gateway: gateway_id.to_string(),
            device: device.to_string(),
            date: date.to_string(),
            events: Vec::new(),
        };
    }
    pub fn gateway_id(&self) -> &String {
        return &self.gateway;
    }

    pub fn device(&self) -> &String {
        return &self.device;
    }

    pub fn date(&self) -> &String {
        return &self.date;
    }

    pub fn events(&self) -> &Vec<EventData> {
        return &self.events;
    }

    pub fn add_events(&mut self, evts: &mut Vec<EventData>) {
        self.events.append(evts);
    }

    pub fn sort_events(&mut self) {
        self.events
            .sort_by(|a, b| return a.date().to_lowercase().cmp(&b.date().to_lowercase()))
    }
}
