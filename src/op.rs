use std::error::Error;

use super::list::List;
use super::list;

use crate::config::*;

#[derive(Debug)]
pub enum Op {
    Update,
    List,
    Help,
    Invalid,
    Create(List),
    Show(List),
    Edit(List),
    Delete(List),
    Remove(List, Vec<usize>),
    Add(List, Vec<String>),
    Search(List, String),
}

impl Op {
    pub fn exec(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Op::Update => list::update_lists(),
            Op::List => list::show_lists(),
            Op::Help =>  {
                println!("{}", HELP_TEXT);
                Ok(())
            },
            Op::Invalid => {
                println!("{}", INVALID_TEXT);
                println!("{}", HELP_TEXT);
                Ok(())
            },
            Op::Create(list) => list.create(),
            Op::Show(list) => list.show(),
            Op::Edit(list) => list.edit(),
            Op::Delete(list) => list.delete(),
            Op::Remove(list, vals) => {
                list.remove(vals)?;
                list.show()?;
                Ok(())
            },
            Op::Add(list, vals) => {
                list.add(vals)?;
                list.show()?;
                Ok(())
            }
            Op::Search(_, _) => Ok(()),
        }
    }
}
