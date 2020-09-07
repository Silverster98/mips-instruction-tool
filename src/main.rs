use structopt::StructOpt;
use std::path::{PathBuf};
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod util;

#[derive(StructOpt, Debug)]
/// mitool, pass `-h`
struct Cli {
    /// subcommand
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// convert binary file to coe format file
    B2c {
        /// input .bin file e.g. -i inst.bin
        #[structopt(short, long, parse(from_os_str))]
        input: Option<PathBuf>,

        /// output .coe file to store hex string[optional] e.g. -o inst.coe
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
    },
    /// convert hex string in coe file to readable instructions 
    C2i {
        /// one hex string, convert one hex string to instruction e.g. -s BFC00000
        #[structopt(short, long)]
        string: Option<String>,

        /// input file, convert hex strings in input file to instructions e.g. -i inst.coe
        #[structopt(short, long, parse(from_os_str), conflicts_with = "string")]
        input: Option<PathBuf>,

        /// output file, store all the translated instructions[optional] e.g. -o inst.S
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
    }
}

enum CMD {
    B2C,
    C2I,
}

/// print mitool usage info
fn print_usage_info(cmd: CMD) {
    match cmd {
        CMD::B2C => {
            println!("USAGE:\n    mitool b2c --input <input> --string <string>");
        },
        CMD::C2I => {
            println!("USAGE:\n    mitool c2i --string <string> --input <input> --output <output>");
        },
    }
}

/// b2c process
/// convert bin file to coe file
fn b2c_process(_input: &Option<PathBuf>, _output: &Option<PathBuf>) -> std::io::Result<()>{
    let input = match _input {
        Some(p) => {
            if p == &PathBuf::from("") {return Err(io::Error::new(io::ErrorKind::Other, "Invalid input!"));}
            p
        },
        None => {return Err(io::Error::new(io::ErrorKind::Other, "Invalid input!"));}
    };

    let t = PathBuf::from(format!("{}.coe", input.file_stem().unwrap().to_str().unwrap()));
    let output = match  _output {
        Some(p) => {p},
        None => {&t},
    };

    let mut fi = File::open(input)?;
    let fo = File::create(output)?;
    {
        (&fo).write_fmt(format_args!("; Generated automatically by mitool written by Rust\n"))?;
        (&fo).write_fmt(format_args!("memory_initialization_radix=16;\n"))?;
        (&fo).write_fmt(format_args!("memory_initialization_vector=\n"))?;
        let mut addr = 0;
        let mut buf = [0; 4];

        loop {
            let n = fi.read(&mut buf)?;
            if n < 4 {break;}
            if addr != 0 {
                (&fo).write_fmt(format_args!(",\n"))?;
            }

            (&fo).write_fmt(format_args!("{:02X}{:02X}{:02X}{:02X}", buf[3], buf[2], buf[1], buf[0]))?;
            addr += 1;
        }
        (&fo).write_fmt(format_args!(";\n"))?;
    }

    Ok(())
}

/// c2i process 
/// convert coe format file to instruction file
fn c2i_process(_string: &Option<String>, _input: &Option<PathBuf>, _output: &Option<PathBuf>) -> std::io::Result<()> {
    let re = regex::Regex::new(r"^[0-9a-fA-F]{8}$").unwrap();

    let is_string = if _string == &None {false} else {true};

    if is_string {
        let string = _string.as_ref().unwrap();
        if !re.is_match(string) {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid input!"));
        }

        println!("{}", util::c2i::hex2instr(string));
    } else {
        let input = match _input {
            Some(p) => {
                if p == &PathBuf::from("") {return Err(io::Error::new(io::ErrorKind::Other, "Invalid input!"));}
                p
            },
            None => {return Err(io::Error::new(io::ErrorKind::Other, "Invalid input!"));}
        };

        let t = PathBuf::from(format!("{}.S", input.file_stem().unwrap().to_str().unwrap()));
        let output = match  _output {
            Some(p) => {p},
            None => {&t},
        };

        let fi = File::open(input)?;
        let fo = File::create(output)?;
        {
            let reader = io::BufReader::new(fi);
            for line in reader.lines() {
                let line = line.unwrap().trim_end_matches(|c| c == ',' || c == ';').to_string();
                
                if re.is_match(&line) {
                    (&fo).write_fmt(format_args!("{}\n", util::c2i::hex2instr(&line)))?;
                } else {
                    (&fo).write_fmt(format_args!("/* {} */\n", line))?;
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::B2c {input, output} => {
            match b2c_process(&input, &output) {
                Ok(_) => {println!("Bin2Coe Done!");},
                Err(e) => {
                    println!("{:?}", e);
                    print_usage_info(CMD::B2C);
                }
            }
        },
        Command::C2i {string, input, output} => {
            match c2i_process(&string, &input, &output) {
                Ok(_) => {println!("Coe2Ins Done!");},
                Err(e) => {
                    println!("{:?}", e);
                    print_usage_info(CMD::C2I);
                }
            }
        },
    }    
}
