extern crate chrono;

use std::error::Error;

use chrono::NaiveDate;

#[derive(Clone)]
#[derive(Debug)]
pub enum ListInstance {
    Main,
    Date(NaiveDate),
}

impl ListInstance {
    pub fn from(s: &str) -> Result<ListInstance, Box<dyn Error>> {
        match s {
            "Main" => Ok(ListInstance::Main),
            date => Ok(ListInstance::Date(
                        NaiveDate::parse_from_str(date, "%Y-%m-%d")?
            )),
        }
    }
}
