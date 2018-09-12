use std::io;
use std::num;
use std::env;
use std::result;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::fmt::{Debug, Display};

type MyResult = result::Result<String, String>;

trait Test {
    fn stub_function();
}

impl Test for MyResult {
    fn stub_function() {

    }
}

trait A {
    fn a() {

    }
}

trait B {
    fn b();
}

trait C: A + B {

}

pub fn start() {
    let doubled_from_file = read_from_file_simple("/home/vladislav/test");
    match doubled_from_file {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Ошибка {:?}", err),
    };
    test_panic();
}

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError)
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn read_from_file<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    File::open(file_path)
        .map_err(CliError::Io)
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(CliError::Io)
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents.trim().parse::<i32>()
                .map_err(CliError::Parse)
                .map(|n| 2 * n)
        })
}

fn read_from_file_simple<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n = try!(contents.trim().parse::<i32>());
    Ok(2 * n)
}

fn read_from_args() {
    let mut args = env::args();
    match read_args(&mut args) {
        Ok(n) => println!("Input arg {}", n),
        Err(err) => println!("Error intput arg {}", err),
    }

    println!("Input something line please:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_res) => { },
        Err(err) => {
            input = "".to_string();
            println!("Error {}", err);
        },
    };

    let res = extension_explicit(&input).unwrap_or("Not found\n");
    print!("{}", res);
    println!("{}", None.unwrap_or_else(|| "ASAP"));

    let file_name2 = "test.txt";
    let res2 = file_path_ext_explicit(file_name2).unwrap_or("Not found");
    println!("{}", res2);


    let mut test_res: MyResult = Ok("Real hip-hop".to_string());
    println!("{}", test_res.unwrap());


    println!("Number {}", "20".parse::<i32>().unwrap());

    let mut test_res: Result<String, String> = Err("Error".to_string());
    println!("{}", test_res.unwrap());
}

fn find_symbol(text: &str, symbol: char) -> Option<usize> {
    for (offset, c) in text.char_indices() {
        if c == symbol {
            return Some(offset)
        }
    };
    None
}

fn extension_explicit(file_name: &str) -> Option<&str> {
    find_symbol(file_name, '.').map(|i| &file_name[i+1..])
}

fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}

fn file_path_ext_explicit(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension_explicit)
}

fn file_name(path: &str) -> Option<&str>{
    if path == ".." || path == "." {
        return None
    }
    Some(path)
}

fn read_args(args: &mut env::Args) -> Result<i32, String> {
    args.nth(1)
        .ok_or("Not arguments!".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
}

fn test_panic() -> ! {
    panic!("End of all")
}
