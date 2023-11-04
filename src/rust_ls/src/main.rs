use clap::{Command, Arg, ArgAction, ValueHint};
use std::{fs, io::{self, Write}};

const NAME: &str = "rust_ls";

enum InputType{
    Directory,
    Pwd,
}

fn app() -> Command{
    Command::new(NAME)
        .version("1.0")
        .about("Ls implementation on Rust")
        .arg(
            Arg::new("DIR")
            .action(ArgAction::Append)
            .value_hint(ValueHint::DirPath)
        )
}

fn main() -> io::Result<()>{
    let matches = app().get_matches();

    let dirs: Vec<String>= match matches.get_many::<String>("DIR"){
        Some(v) => v.map(|v| v.to_owned()).collect(),
        None => vec!["-".to_owned()],
    };

    for dir in dirs{
        match dir_type(&dir) {
            InputType::Pwd => print_dir(".").unwrap_or_else(|e| println!("{}: {}", NAME, e)),
            InputType::Directory => print_dir(&dir).unwrap_or_else(|e| println!("{}: {}", NAME, e)),
        }
    }
    
    Ok(())
 
}

fn dir_type(path: &str) -> InputType{
    if path == "-"{
        InputType::Pwd
    } else {
        InputType::Directory
    }
}

fn print_dir(path: &str) -> io::Result<()>{
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    
    let mut stdout = io::stdout().lock();

    for file in entries{
        if let Some(res) = file.file_name(){
            if let Some(res) = res.to_str(){
                write!(&mut stdout, "{} ", res)?;
            }
        }
    }
    write!(&mut stdout, "\n")?;
    Ok(())
}

