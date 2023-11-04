use clap::{Command, Arg,ArgAction};

const NAME: &str = "rust_echo";
fn app() -> Command{
    Command::new(NAME)
    .version("1.0")
    .about("Echo implementation in Rust")
    .arg(Arg::new("String").action(ArgAction::Append))
}

fn execute(value: &Vec<String>) {
    for input in value.iter(){
        println!("{}", input);
    }
}
fn main() {
    let app = app().get_matches();

    let value: Vec<String> = match app.get_many::<String>("String") {
        Some(s) => s.map(|s| s.to_owned()).collect(),
        None => vec![String::new()],
    };
    execute(&value);
    
}
