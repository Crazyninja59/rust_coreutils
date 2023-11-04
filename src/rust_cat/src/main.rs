use clap::{Command,Arg,ArgAction,ValueHint};
use std::fs;
use std::io::{self,Write};


const NAME: &str = "rust_cat";

fn app() -> Command{
    Command::new(NAME)
        .version("1.0")
        .about("Cat implementaion in Rust")
        .arg(
            Arg::new("FILE")
            .action(ArgAction::Append)
            .value_hint(ValueHint::FilePath)
        )
    

}
fn main() {
    let mathes = app().get_matches();
    let files: Vec<String> = match mathes.get_many::<String>("FILE"){
        Some(v) => v.map(|v| v.to_owned()).collect(),
        None => vec!["NONE".to_owned()],
    };
    let mut error_messages: Vec<String> = Vec::new();
    for path in files{
        if let Err(e) = write_to_console(&path){
            error_messages.push(format!("{}: {}", path, e));
        }
    }
    
    if !error_messages.is_empty(){
        let line_joiner = format!("{}: ", NAME);
        for error in error_messages{
            println!("{}{}",line_joiner, error);
        }
        
    }
}

fn write_to_console(path: &str) -> io::Result<()>{
    
    let content = fs::read(path)?;

    let mut stdout = io::stdout().lock();
    
    stdout.write_all(&content)?;
    stdout.write_all(b"\n")?;
    
    Ok(())

}
