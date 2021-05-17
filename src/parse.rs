use chrono::DateTime;

use std::error::Error;

use super::list;
use super::op::Op;
use list::{List, ListInstance, ListType};

use chrono::{Duration, Local};

#[derive(Debug)]
pub enum OpType {
    Update,
    List,
    Create,
    Delete,
    Show,
    Edit,
    Add,
    Remove,
    Help,
    Invalid,
    Search,
}

enum State {
    Start,
    Update,
    Help,
    Invalid,
    CreateNormal,
    CreateTyped,
    Show,
    List,
    Edit,
    Add,
    Remove,
    Delete,
    Search,
    ShowTomorrow,
    ShowTemplate,
    GetDate,
    Date,
    ShowDate,
    NoListDir,
    GetString,
    GetNumber,
}

impl State {
    fn next(self, input: &str) -> State {
        match self {
            State::GetString => State::GetString,
            State::GetNumber => {
                if input.parse::<usize>().is_ok() {
                    State::GetNumber
                } else {
                    State::Invalid
                }
            },
            State::NoListDir => State::NoListDir,
            State::Update => State::Invalid,
            State::List => State::Invalid,
            State::Start => match input {
                "--help" => State::Help,
                "ls" => State::List,
                "update" => State::Update,
                _ => {
                    let list_name = input;
                    let list_dir = match list::list_dir() {
                        Ok(list_dir) => list_dir,
                        _ => {
                            return State::NoListDir
                        }
                    };
                    if list::list_exists(list_dir, list_name) {
                        return State::Show
                    } else { // Does not exist
                        return State::CreateNormal
                    };
                },
            },
            State::Help => State::Invalid,
            State::Invalid => State::Invalid,
            State::CreateNormal => State::CreateTyped,
            State::CreateTyped => State::Invalid,
            State::Show => match input {
                "e" => State::Edit,
                "a" => State::Add,
                "x" => State::Remove,
                "s" => State::Search,
                "t" => State::ShowTomorrow,
                "d" => State::Date,
                "m" => State::ShowTemplate,
                "delete" => State::Delete,
                _ => State::Invalid,
            }
            State::Edit => State::Invalid,
            State::Delete => State::Invalid,
            State::Add => State::GetString,
            State::Remove => State::GetNumber,
            State::Search => State::GetString,
            State::ShowTomorrow => match input {
                "e" => State::Edit,
                "a" => State::Add,
                "x" => State::Remove,
                "s" => State::Search,
                _ => State::Invalid,
            },
            State::ShowTemplate => match input {
                "e" => State::Edit,
                "a" => State::Add,
                "x" => State::Remove,
                "s" => State::Search,
                _ => State::Invalid,
            },
            State::Date => State::GetDate,
            State::GetDate => match DateTime::parse_from_str(input, "%Y-%m-%d") {
                Ok(_) => State::ShowDate,
                _ => State::Invalid,
            }
            State::ShowDate => match input {
                "e" => State::Edit,
                "a" => State::Add,
                "x" => State::Remove,
                "s" => State::Search,
                _ => State::Invalid,
            },
        }
    }

}

pub fn parse(args: Vec<String>) -> Op {
    let mut op = OpType::Help;
    let mut list: Option<String> = None;
    let mut typ: Option<ListType> = None;
    let mut instance: Option<ListInstance> = None;
    let mut vals = Vec::new();

    let mut state = State::Start;

    let args: Vec<String> = args.into_iter().skip(1).collect();

    for arg in &args {
        state = state.next(arg);
        match state {
            State::List => op = OpType::List,
            State::Start => (),
            State::NoListDir => op = OpType::Invalid,
            State::Help => op = OpType::Help,
            State::Update => op = OpType::Update,
            State::Invalid => op = OpType::Invalid,
            State::CreateNormal => {
                op = OpType::Create;
                list = Some(arg.to_string());
                typ = Some(ListType::Normal);
                instance = Some(ListInstance::Main);
            },
            State::CreateTyped => {
                instance = Some(ListInstance::Main);
                typ = match ListType::from(arg) {
                    Ok(typ) => Some(typ),
                    _ => {
                        op = OpType::Invalid;
                        break;
                    }
                };
            },
            State::Show => {
                op = OpType::Show;
                list = Some(arg.to_string());
                typ = Some(ListType::Normal);
                instance = Some(ListInstance::Main);
            },
            State::Edit => op = OpType::Edit,
            State::Add => op = OpType::Add,
            State::Remove => op = OpType::Remove,
            State::Delete => op = OpType::Delete,
            State::Date => (),
            State::GetString => vals.push(arg.to_string()),
            State::GetNumber => vals.push(arg.to_string()),
            State::GetDate => instance = match ListInstance::from(arg) {
                Ok(instance) => Some(instance),
                _ => {
                    op = OpType::Invalid;
                    break;
                },
            },
            State::Search => op = OpType::Search,
            State::ShowTomorrow => instance = Some(
                ListInstance::Date(
                    (Local::today() + Duration::days(1)).naive_local()
                )
            ),
            State::ShowTemplate => instance = Some(ListInstance::Main),
            State::ShowDate => (),
        }
    }

    match op {
        OpType::Update => Op::Update,
        OpType::List => Op::List,
        OpType::Help => Op::Help,
        OpType::Invalid => Op::Invalid,
        op => {
            let list = List {
                name: list.unwrap(),
                typ: typ.unwrap(),
                instance: instance.unwrap(),
            };

            match op {
                OpType::Create => Op::Create(list),
                OpType::Show => Op::Show(list),
                OpType::Edit => Op::Edit(list),
                OpType::Delete => Op::Delete(list),
                OpType::Remove => Op::Remove(list, match validate_all_usize(&vals) {
                    Ok(vals) => vals,
                    _ => return Op::Invalid,
                }),
                OpType::Add => Op::Add(list, vals),
                OpType::Search => Op::Search(list, vals.join(" ")),
                _ => Op::Invalid,
            }
        }
    }
}

fn validate_all_usize(
    vals: &Vec<String>
) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut ret: Vec<usize> = Vec::new();
    for val in vals.iter() {
        let val = val.parse::<usize>()?;
        ret.push(val);
    }

    Ok(ret)
}
