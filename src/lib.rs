use std::{collections::HashMap, error::Error};

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let path = args[1].clone();

        Ok(Config { path })
    }
}

fn translate_op(operation: &str) -> Result<&str, &'static str> {
    match operation {
        "0" => Ok("0101010"),
        "1" => Ok("0111111"),
        "-1" => Ok("0111010"),
        "D" => Ok("0001100"),
        "A" => Ok("0110000"),
        "!D" => Ok("0001101"),
        "!A" => Ok("0110001"),
        "-D" => Ok("0001111"),
        "-A" => Ok("0110011"),
        "D+1" => Ok("0011111"),
        "A+1" => Ok("0110111"),
        "D-1" => Ok("0001110"),
        "A-1" => Ok("0110010"),
        "D+A" => Ok("0000010"),
        "D-A" => Ok("0010011"),
        "A-D" => Ok("0000111"),
        "D&A" => Ok("0000000"),
        "D|A" => Ok("0010101"),
        "M" => Ok("1110000"),
        "!M" => Ok("1110001"),
        "-M" => Ok("1110011"),
        "M+1" => Ok("1110111"),
        "M-1" => Ok("1110010"),
        "D+M" => Ok("1000010"),
        "D-M" => Ok("1010011"),
        "M-D" => Ok("1000111"),
        "D&M" => Ok("1000000"),
        "D|M" => Ok("1010101"),
        _ => {
            println!("Unknown operation: {}", operation);
            Err("Error while parsing operation")
        }
    }
}

fn translate_dest(destination: &str) -> Result<&str, &'static str> {
    match destination {
        "NULL" => Ok("000"),
        "M" => Ok("001"),
        "D" => Ok("010"),
        "A" => Ok("100"),
        "MD" => Ok("011"),
        "AM" => Ok("101"),
        "AD" => Ok("110"),
        "AMD" => Ok("111"),
        _ => Err("Destination not found"),
    }
}

fn translate_jmp(jump: &str) -> Result<&str, &'static str> {
    match jump {
        "NULL" => Ok("000"),
        "JGT" => Ok("001"),
        "JEQ" => Ok("010"),
        "JLT" => Ok("100"),
        "JGE" => Ok("011"),
        "JNE" => Ok("101"),
        "JLE" => Ok("110"),
        "JMP" => Ok("111"),
        _ => {
            println!("Trying to tranlsate: {}", jump);
            Err("Unknown jump")
        }
    }
}

fn normalize(content: &String) -> String {
    let mut norm = String::new();

    for line in content.lines() {
        let mut trim = line.trim().to_string();

        if trim.starts_with("//") || trim.is_empty() {
            continue;
        }

        trim.retain(|ch| !ch.is_whitespace());

        if let Some(ix) = trim.find("//") {
            trim = trim.chars().take(ix).collect::<String>();
        }

        if !trim.starts_with("@") && !trim.starts_with('(') {
            if !trim.contains("=") {
                let mut temp = String::from("NULL=");
                temp.push_str(&trim);
                trim = temp;
            }
            if !trim.contains(";") {
                trim.push_str(";NULL")
            }
        }

        norm.push_str(&trim);
        norm.push('\n');
    }

    norm
}

fn parse_a(line: &str, table: &mut SymbolTable) -> Result<String, Box<dyn Error>> {
    let n: i32;

    if !line.chars().nth(1).unwrap().is_numeric() {
        let key = &line.chars().skip(1).collect::<String>();
        if let Some(num) = table.address.get(key) {
            n = *num;
        } else {
            n = table.insert(key, None).unwrap();
        }
    } else {
        n = line[1..].parse::<i32>()?;
    }

    let bin = format!("{n:b}");

    Ok(format!("{bin:0>16}"))
}

fn parse_c(line: &str) -> Result<String, Box<dyn Error>> {

    let mut result = String::from("111");
    let mut iter = line.split("=");
    let dest = iter.next().unwrap();
    let mut right = iter.next().unwrap().split(";");
    let op = right.next().unwrap();
    let jmp = right.next().unwrap();

    result.push_str(&translate_op(op)?);
    result.push_str(&translate_dest(dest)?);
    result.push_str(&translate_jmp(jmp)?);

    Ok(result)
}

fn parse_l(line: &str, n: i32, table: &mut SymbolTable) -> Result<(), Box<dyn Error>> {
    if let Some(ix) = line.find(')') {
        table.insert(&line[1..ix].to_string(), Some(n))?;
    }
    Ok(())
}

pub fn parse(content: &String, table: &mut SymbolTable) -> Result<String, Box<dyn Error>> {
    let norm = normalize(content);
    let mut parsed = String::new();
    let mut ix = 0;
    let mut no_labels = String::new();

    // First pass for handling labels
    for line in norm.lines() {
        if line.starts_with("(") {
            parse_l(line, ix, table)?;
        } else {
            no_labels.push_str(line);
            no_labels.push('\n');
            ix += 1;
        }
    }

    // Second pass for handling instructions
    for line in no_labels.lines() {
        if line.starts_with("@") {
            // A-type instructions
            let p = parse_a(line, table)?;
            parsed.push_str(&p);
            ix += 1;
            parsed.push('\n');
        } else {
            // C-type instruction
            let p = parse_c(line)?;
            parsed.push_str(&p);
            ix += 1;
            parsed.push('\n');
        }
    }

    Ok(parsed)
}

pub struct SymbolTable {
    address: HashMap<String, i32>,
    ix: i32,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            address: HashMap::from([
                ("R0".to_string(), 0),
                ("R1".to_string(), 1),
                ("R2".to_string(), 2),
                ("R3".to_string(), 3),
                ("R4".to_string(), 4),
                ("R5".to_string(), 5),
                ("R6".to_string(), 6),
                ("R7".to_string(), 7),
                ("R8".to_string(), 8),
                ("R9".to_string(), 9),
                ("R10".to_string(), 10),
                ("R11".to_string(), 11),
                ("R12".to_string(), 12),
                ("R13".to_string(), 13),
                ("R14".to_string(), 14),
                ("R15".to_string(), 15),
                ("SCREEN".to_string(), 16384),
                ("KBD".to_string(), 24576),
                ("SP".to_string(), 0),
                ("LCL".to_string(), 1),
                ("ARG".to_string(), 2),
                ("THIS".to_string(), 3),
                ("THAT".to_string(), 4),
            ]),

            ix: 16,
        }
    }

    fn insert(&mut self, element: &String, n: Option<i32>) -> Result<i32, &'static str> {
        if let Some(ix) = n {
            self.address.insert(element.to_owned(), ix);
            return Ok(ix);
        } else if self.ix < 16384 {
            self.address.insert(element.to_owned(), self.ix);
            self.ix += 1;
            return Ok(self.ix - 1);
        }
        Err("Not enough space in RAM")
    }
}
