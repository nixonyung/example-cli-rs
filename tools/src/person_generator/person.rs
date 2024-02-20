#[derive(Debug)]
pub struct Address {
    pub room: String,
    pub building: String,
    pub street: String,
    pub district: String,
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    // (ref.) [Rust Chrono parse date String, ParseError(NotEnough) and ParseError(TooShort)](https://stackoverflow.com/questions/61179070/rust-chrono-parse-date-string-parseerrornotenough-and-parseerrortooshort)
    pub birthday: chrono::NaiveDate,
    pub address: Address,
}

impl Person {
    pub fn age(&self) -> u32 {
        chrono::Utc::now()
            .date_naive()
            .years_since(self.birthday)
            .unwrap()
    }
}
