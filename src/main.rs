use hack_assembler::{parse, Config, SymbolTable};
use std::{env, fs, fs::File, io::Read, path::PathBuf, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    let mut f_path = PathBuf::from(&config.path);

    let mut file = File::open(&f_path).unwrap_or_else(|err| {
        println!("Error while opening file: {}", err);
        process::exit(1);
    });

    let mut content: String = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => {
            println!("Successfully read file.");
        }
        Err(e) => {
            println!("Error while reading file: {}", e);
            process::exit(1);
        }
    }

    f_path.set_extension("hack");

    let mut table = SymbolTable::new();

    match parse(&content, &mut table) {
        Ok(parsed) => match fs::write(
            f_path.to_str().expect("Error while getting path to save"),
            &parsed,
        ) {
            Ok(_) => println!("Successfully saved file: {}", &f_path.to_string_lossy()),
            Err(_) => {
                println!("Error occured while writing to file");
                process::exit(1);
            }
        },
        Err(err) => {
            println!("Error occured: {}", err);
            process::exit(1);
        }
    }
}
