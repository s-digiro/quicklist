use std::path::PathBuf;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::prelude::*;
use std::fs::OpenOptions;

use chrono::{NaiveDate, Local, Duration};
use left_pad::leftpad;
use termion::color;

use crate::config::*;

mod list_instance;
mod list_type;
mod meta_data;

pub use list_instance::ListInstance;
pub use list_type::ListType;
pub use meta_data::MetaData;

#[derive(Debug)]
pub struct List {
    pub name: String,
    pub typ: ListType,
    pub instance: ListInstance,
}

impl List {
    pub fn get(name: &str) -> Result<List, Box<dyn Error>> {
        let meta = MetaData::within(name)?;
        match meta.typ {
            ListType::Normal => List::get_main(name),
            ListType::RepeatsDaily => List::get_today(name),
            ListType::ExistsDaily => List::get_today(name),
        }
    }

    pub fn get_today(name: &str) -> Result<List, Box<dyn Error>> {
        let meta = MetaData::within(name)?;
        let ret = List {
            name: name.to_string(),
            typ: meta.typ,
            instance: ListInstance::Date(Local::today().naive_local()),
        };

        if ret.exists()? {
            Ok(ret)
        } else {
            bail!("{} cannot be found for today", name)
        }
    }

    pub fn get_date(name: &str, date: NaiveDate) -> Result<List, Box<dyn Error>> {
        let meta = MetaData::within(name)?;
        let ret = List {
            name: name.to_string(),
            typ: meta.typ,
            instance: ListInstance::Date(date),
        };

        if ret.exists()? {
            Ok(ret)
        } else {
            bail!("{} cannot be found for date {}", name, date)
        }
    }

    pub fn get_main(name: &str) -> Result<List, Box<dyn Error>> {
        let meta = MetaData::within(name)?;
        let ret = List {
            name: name.to_string(),
            typ: meta.typ,
            instance: ListInstance::Main,
        };

        if ret.exists()? {
            Ok(ret)
        } else {
            bail!("{} cannot be found", name)
        }
    }

    pub fn meta_path(&self) -> Result<PathBuf, Box<dyn Error>> {
        meta_path(&self.name)
    }

    pub fn path(&self) -> Result<PathBuf, Box<dyn Error>> {
        let meta = self.meta()?;

        let suffix = match meta.typ {
            ListType::RepeatsDaily | ListType::ExistsDaily =>  {
                match &self.instance {
                    ListInstance::Date(s) => format!("-{}", s),
                    ListInstance::Main => "".to_string(),
                }
            },
            ListType::Normal => String::new(),
        };

        let path = if let ListType::Normal = meta.typ {
            meta.files.get(0).expect(&format!("MetaData for Normal list {:?} is missing a path", self)).clone()
        } else {
            let mut dir = list_dir()?;
            let path = format!("{}{}.txt", self.name, suffix);
            dir.push(path);
            dir
        };

        Ok(path)
    }

    pub fn meta(&self) -> Result<MetaData, Box<dyn Error>> {
        MetaData::of(self)
    }

    pub fn delete(&self) -> Result<(), Box<dyn Error>> {
        use std::io;

        print!("Are you sure you want to delete quicklist '{}'? [(Y)es/(N)o]: ", self.name);
        io::stdout().flush().unwrap();

        let input: Option<char> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as char);

        if format!("{}", input.unwrap()).to_lowercase() == "y" {
            let meta = self.meta()?;
            let dir_entries = fs::read_dir(list_dir()?)?;

            for dir_entry in dir_entries {
                let path = dir_entry?.path();
                if meta.files.contains(&path) {
                    fs::remove_file(path)?;
                }
            }

            fs::remove_file(self.meta_path()?)?;
            println!("Deleted quicklist '{}'", self.name);
        } else {
            println!("Aborting!");
        }

        Ok(())
    }

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        if self.meta_exists()? {
            println!("exists");
            let mut meta = OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.meta_path()?)?;
            meta.write_all(
                &format!(
                    "\n{}",
                    self.path()?.into_os_string().into_string().unwrap()
                ).as_bytes()
            )?;

            File::create(self.path()?)?;
        } else {
            let mut meta = File::create(self.meta_path()?)?;
            meta.write_all(&format!("{}\n", self.typ.to_string()).as_bytes())?;
            meta.write_all(
                &format!(
                    "{}",
                    self.path()?.into_os_string().into_string().unwrap()
                ).as_bytes()
            )?;

            File::create(self.path()?)?;
            println!("Created new quicklist '{}'", self.name);
        };


        Ok(())
    }

    pub fn instance(&self, date: NaiveDate) -> List {
        List {
            name: self.name.clone(),
            typ: self.typ,
            instance: ListInstance::Date(date),
        }
    }

    pub fn edit(&self) -> Result<(), Box<dyn Error>> {
        let list_path = self.path()?;
        if list_path.extension().unwrap() == "sc" {
            subprocess::Exec::cmd("sc-im").arg(list_path).join()?;
        } else {
            subprocess::Exec::cmd(EDITOR).arg(list_path).join()?;
        }

        Ok(())
    }

    pub fn add(&self, vals: &Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.path()?)?;

        for line in vals {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    pub fn search(&self, val: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(self.path()?)?;

        let mut lines = Vec::new();

        lines.extend(
            BufReader::new(file)
                .lines()
                .filter(|line| match line {
                    Ok(_) => true,
                    _ => false,
                })
                .map(|line| line.unwrap().to_string())
        );

        let digits = lines.len().to_string().len();

        let mut i = 1;
        for line in lines {
            let val = val.to_lowercase();
            if line.to_lowercase().contains(&val) {
                println!(
                    " {}{}{} {}",
                    color::Fg(color::Yellow),
                    leftpad(i.to_string(), digits),
                    color::Fg(color::Reset),
                    line
                );
            }
            i = i + 1;
        }

        Ok(())
    }

    pub fn show(&self) -> Result<(), Box<dyn Error>> {
        let file = File::open(self.path()?)?;

        let mut lines = Vec::new();

        lines.extend(
            BufReader::new(file)
                .lines()
                .filter(|line| match line {
                    Ok(_) => true,
                    _ => false,
                })
                .map(|line| line.unwrap().to_string())
        );

        let digits = lines.len().to_string().len();

        let mut i = 1;
        for line in lines {
            println!(
                " {}{}{} {}",
                color::Fg(color::Yellow),
                leftpad(i.to_string(), digits),
                color::Fg(color::Reset),
                line
            );
            i = i + 1;
        }

        Ok(())
    }

    pub fn remove(&self, vals: &Vec<usize>) -> Result<(), Box<dyn Error>> {
        let mut new_contents = String::new();

        {
            let file = File::open(self.path()?)?;

            let lines = BufReader::new(file).lines();
            for (i, line) in lines.enumerate() {
                let i = i + 1;
                if !vals.contains(&i) {
                    new_contents.push_str(&format!("{}\n", line?));
                }
            }
        }

        fs::write(self.path()?, new_contents)?;

        Ok(())
    }

    pub fn meta_exists(&self) -> Result<bool, Box<dyn Error>> {
        let dir = list_dir()?;
        Ok(meta_exists(dir, &self.name))
    }

    pub fn exists(&self) -> Result<bool, Box<dyn Error>> {
        let dir = list_dir()?;
        let name = match self.instance {
            ListInstance::Main => self.name.clone(),
            ListInstance::Date(d) => format!("{}-{}", &self.name, d),
        };
        Ok(list_exists(dir, &name))
    }
}

pub fn show_lists() -> Result<(), Box<dyn Error>> {
    let mut lists = all_list_names()?;
    lists.sort();
    for list in lists {
        println!(
            " {}{}",
            color::Fg(color::Yellow),
            list
        );
    }

    Ok(())
}

pub fn list_exists(mut list_dir: PathBuf, list_name: &str) -> bool {
    list_dir.push(format!("{}.txt", list_name));

    if list_dir.exists() {
        return true
    }

    list_dir.pop();
    list_dir.push(format!("{}.sc", list_name));

    list_dir.exists()
}

pub fn meta_exists(mut list_dir: PathBuf, list_name: &str) -> bool {
    list_dir.push(format!("{}.meta", list_name));
    list_dir.exists()
}

pub fn list_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut list_dir = dirs::data_local_dir().unwrap();
    list_dir.push(PROGRAM_NAME);

    fs::create_dir_all(list_dir.clone())?;

    Ok(list_dir)
}

pub fn all_list_names() -> Result<Vec<String>, Box<dyn Error>> {
    let mut ret = Vec::new();

    let dir_entries = fs::read_dir(list_dir()?)?;
    for dir_entry in dir_entries {
        let path = dir_entry?.path();
        if path.display().to_string().ends_with(".meta") {
            let mut name = path.file_name().unwrap().to_str().unwrap().to_string();
            name.truncate(name.len() - ".meta".len());
            ret.push(name);
        }
    }

    Ok(ret)
}

pub fn update_lists() -> Result<(), Box<dyn Error>> {
    for name in all_list_names()? {
        let list = List::get_main(&name)?;
        match list.meta()?.typ {
            ListType::ExistsDaily => {
                let todays = list.instance(Local::today().naive_local());
                let tomorrows = list.instance(
                    (Local::today() + Duration::days(1)
                ).naive_local());

                if !todays.exists()? {
                    todays.create()?;
                }
                if !tomorrows.exists()? {
                    tomorrows.create()?;
                }
            },
            ListType::RepeatsDaily => {
                let todays = list.instance(Local::today().naive_local());
                let tomorrows = list.instance(
                    (Local::today() + Duration::days(1)
                ).naive_local());

                let template_contents = fs::read_to_string(list.path()?)?;

                if !todays.exists()? {
                    todays.create()?;

                    fs::write(todays.path()?, template_contents.clone())?;
                }
                if !tomorrows.exists()? {
                    tomorrows.create()?;

                    fs::write(tomorrows.path()?, template_contents)?;
                }
            },
            ListType::Normal => (),
        }
    }

    Ok(())
}

pub fn meta_path(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut dir = list_dir()?;
    dir.push(format!("{}.meta", name));
    Ok(dir)
}
