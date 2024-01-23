use crate::error::ParsingError;

#[derive(Clone)]
pub struct EventData {
    date: String,
    battery: String,
    value: String,
}

impl TryFrom<String> for EventData {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, ParsingError> {
        let splitted = value.split(";").collect::<Vec<&str>>();
        let date = match splitted.get(0) {
            Some(d) => d,
            None => return Err(ParsingError::new("No date found")),
        };
        let battery = match splitted.get(1) {
            Some(d) => d,
            None => return Err(ParsingError::new("No battery found")),
        };
        let value = match splitted.get(2) {
            Some(d) => d,
            None => return Err(ParsingError::new("No value found")),
        };
        return Ok(Self {
            date: date.to_string(),
            battery: battery.to_string(),
            value: value.to_string(),
        });
    }
}

impl EventData {
    pub fn date(&self) -> String {
        return self.date.clone();
    }
    pub fn battery(&self) -> String {
        return self.battery.clone();
    }
    pub fn value(&self) -> String {
        return self.value.clone();
    }

    pub fn add_date(&mut self, date: &str) {
        self.date = format!("{} {}", date, self.date);
    }
}
