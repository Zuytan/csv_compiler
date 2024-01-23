use std::{collections::HashMap, fs};

use crate::{error::ParsingError, input_file::InputFile};

pub struct InputFileReader {
    files: Vec<InputFile>,
}

impl TryFrom<String> for InputFileReader {
    type Error = ParsingError;

    fn try_from(folder_path: String) -> Result<Self, Self::Error> {
        let folder = match fs::read_dir(folder_path) {
            Ok(p) => p,
            Err(_) => {
                return Err(ParsingError::new("The folder doesn't exists"));
            }
        };
        let mut files = Vec::new();
        for file in folder {
            if file.is_ok() {
                files.push(InputFile::try_from(file.unwrap().path())?);
            } else {
                return Err(ParsingError::new(&format!(
                    "An error occured while parsing file : {}",
                    file.unwrap_err().to_string()
                )));
            }
        }
        return Ok(Self { files });
    }
}

impl InputFileReader {
    pub fn files(&self) -> &Vec<InputFile> {
        return &self.files;
    }

    pub fn get_events_by_device(&self) -> HashMap<String, InputFile> {
        let mut comps: HashMap<String, InputFile> = HashMap::new();
        for comp in self.files.clone() {
            match comps.get_mut(comp.device()) {
                Some(c) => {
                    c.add_events(&mut comp.events().clone());
                }
                None => {
                    let mut c = InputFile::new(&comp.gateway_id(), &comp.device(), &comp.date());
                    c.add_events(&mut comp.events().clone());
                    comps.insert(comp.device().to_string(), c);
                }
            }
        }
        for comp in comps.iter_mut() {
            comp.1.sort_events()
        }
        comps
    }
}
