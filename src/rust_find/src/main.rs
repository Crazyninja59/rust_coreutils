use clap::{Command, Arg, ValueHint};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;


const NAME: &str = "rust_find";


fn app() -> Command{
    Command::new(NAME)
        .version("1.0")
        .about("Find implementation on Rust")
        .arg(
            Arg::new("DIR")
            .value_hint(ValueHint::DirPath)
        )
        .arg(
            Arg::new("Target")
        )
        
}

fn main() {
    let matches = app().get_matches();

    let start_path = match matches.get_one::<String>("DIR"){
        Some(v) => PathBuf::from(v),
        None => PathBuf::from("/home/"),
    };
    let target = match matches.get_one::<String>("Target"){
        Some(v) => v.to_owned(),
        None => String::new(),
    };
    match find_target(&start_path, &target){
        Some(v) => println!("{}", v.display()),
        None => println!("{}: Цель не найдена", NAME)
    }
}

fn find_target(path: &Path, target: &str) -> Option<PathBuf>{
    for entry in  WalkDir::new(path).into_iter().filter_map(|e| e.ok()){
        let path = entry.path();
        println!("{}", path.display());

        if path.is_file() && path.file_name().and_then(|f| f.to_str() ) == Some(target){
            return Some(path.to_path_buf());
        }
        
    }
    None
}
