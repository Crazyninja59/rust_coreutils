use clap::{Command, Arg, ArgAction};
use std::io::Write;
use std::path::PathBuf;
use std::{process, io};


const NAME: &str = "rust_coreutils";

fn app() -> Command{
    Command::new(NAME)
        .about("Rust GNU coreutils")
        .version("1.0")
        .arg(
            Arg::new("utility")
                .required(true)
        )
        .arg(
            Arg::new("arguments")
                .action(ArgAction::Append)
                
        )

}

fn binary_path(utility_name: &str) -> PathBuf{
    let path = format!("src/{u}/target/release/{u}", u = utility_name);
    PathBuf::from(path)
}
fn main() -> io::Result<()> {
    let matches = app().get_matches();
    let utility_name = matches.get_one::<String>("utility").unwrap();
    let args: Vec<_> = match matches.get_many::<String>("arguments"){
        Some(v) => v.map(|x| x.to_owned()).collect(),
        None => vec![],

    };
    let binary = binary_path(utility_name);

    let output = process::Command::new(binary)
        .args(args)
        .output();
        
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match output {
        Ok(v) => handle.write_all(&v.stdout),
        Err(_) => {
            let error = format!("{}: Utility nof found\n",NAME);
            handle.write_all(error.as_bytes())
        }
    }
        
    
}
