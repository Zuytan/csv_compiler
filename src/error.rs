pub struct ParsingError {
    reason: String,
}

impl ParsingError {
    pub fn new(reason: &str) -> Self {
        return ParsingError {
            reason: reason.to_owned(),
        };
    }

    pub fn reason(&self) -> String {
        return self.reason.clone();
    }
}
