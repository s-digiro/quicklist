extern crate simple_error;

use std::error::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};

use super::List;
use super::ListType;

pub struct MetaData {
    pub typ: ListType,
    pub files: Vec<PathBuf>,
}

impl MetaData {
    pub fn within(name: &str) -> Result<MetaData, Box<dyn Error>> {
        let mut typ = None;
        let mut files = Vec::new();

        let meta_path = super::meta_path(name)?;
        let file = File::open(meta_path)?;

        let buf_reader = BufReader::new(file);

        let mut first = true;
        for line in buf_reader.lines() {
            if first {
                typ = Some(ListType::from(&line?)?);
                first = false;
            } else {
                files.push(PathBuf::from(line?));
            }
        }

        match typ {
            Some(typ) => Ok(MetaData { typ, files }),
            None => bail!("MetaData for {} has invalid List Type", name),
        }
    }

    pub fn of(list: &List) -> Result<MetaData, Box<dyn Error>> {
        MetaData::within(&list.name)
    }
}
