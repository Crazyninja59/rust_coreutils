use clap::{Command,Arg, ValueHint};
use std::{path::PathBuf, fs, io};

const NAME: &str = "rust_grep";

fn app() -> Command{
    Command::new(NAME)
        .version("1.0")
        .about("Grep implementation on Rust")
        .arg(
            Arg::new("target")
            .required(true)
        )
        .arg(
            Arg::new("file_path")
            .required(true)
            .value_hint(ValueHint::FilePath)
        )
        
}

fn main() {
    let matches = app().get_matches();
    
    let target = match matches.get_one::<String>("target"){
        Some(v) => v.to_owned(),
        None => String::new(),
    };

    let path = match matches.get_one::<String>("file_path"){
        Some(v) => PathBuf::from(v),
        None => PathBuf::new(),
    };
    match search(&target, path) {
        Ok(v) => v.iter().for_each(|x| println!("{}", x)),
        Err(e) => println!("{}: {}",NAME,e)
        
    }


}

fn search(target: &str, path: PathBuf) -> io::Result<Vec<String>>{
    let contents = fs::read_to_string(path)?;
    
    let result = contents
        .lines()
        .filter(|line| line.contains(target))
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    Ok(result)

}