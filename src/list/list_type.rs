extern crate simple_error;
use std::error::Error;

#[derive(Debug, Copy, Clone)]
pub enum ListType {
    RepeatsDaily,
    ExistsDaily,
    Normal,
}

impl ListType {
    pub fn from(s: &str) -> Result<ListType, Box<dyn Error>> {
        match s {
            "RepeatsDaily" => Ok(ListType::RepeatsDaily),
            "ExistsDaily" => Ok(ListType::ExistsDaily),
            "Normal" => Ok(ListType::Normal),
            _ => bail!("{} cannot be parsed in a valid ListType", s),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ListType::RepeatsDaily => "RepeatsDaily".to_string(),
            ListType::ExistsDaily => "ExistsDaily".to_string(),
            ListType::Normal => "Normal".to_string(),
        }
    }
}
