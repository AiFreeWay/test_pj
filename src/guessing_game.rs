extern crate rand;

use std::io;
use self::rand::Rng;
use std::cmp::Ordering;

pub fn start() {
    println!("Мы загадали для Вас чило, попробуйте его отгадать.");

    loop {
        let input = get_user_input();
        let secret = rand::thread_rng().gen_range(1, 11);

        if compare(input, secret) {
            break;
        }
    }
}

fn get_user_input() -> String {
    println!("Введите число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .ok()
        .expect("input error!!!");

    return input
}

fn compare(input: String, secret: u32) -> bool {
    //let input: u32 = input.trim().parse()
    //    .ok()
    //    .expect("Некорректное число");


    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Это не число!");
            return false;
        },
    };

    match input.cmp(&secret) {
        Ordering::Less => println!("Маленькое {}", secret),
        Ordering::Greater => println!("Большое {}", secret),
        Ordering::Equal => {
            println!("Вы угадали!!!");
            return true;
        }
    }

    return false;
}
