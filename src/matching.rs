#[derive(Debug)]
struct Person {
    name: Option<String>,
}

enum ABenum {
    A(u32),
    B(u64),
}

struct AB {
    a: ABenum,
    b: ABenum,
}

struct ABCX(u32, u32, i32, u64, i64);

enum ABCXenum {
    A(ABCX),
    B,
}

enum OptionalInt {
    Value(i32),
    Missing,
}

pub fn start() {
    let ab = AB { a: ABenum::A(1u32), b: ABenum::B(2u64) };
    let ab2 = AB { a: ABenum::A(3u32), b: ABenum::A(3u32) };

    match ab {
        AB { a: ABenum::A(a), b: ABenum::B(_) } => println!("middle"),
        _ => println!("ignore"),
    }

    match ab2 {
        AB { a: ABenum::A(a), b: ABenum::A(b) } => println!("success"),
        _ => println!("ignore"),
    }
    let abcx = (1u32, 2u32, 3i32, 4u64, 5i64);
    let (a, _, b, _, c) = abcx;

    let abcx = (1u32, 2u32, 3i32, 4u64, 5i64);
    let (a, ..) = abcx;

    let abcx = (1u32, 2u32, 3i32, 4u64, 5i64);
    let (.., c) = abcx;

    let abcx = (1u32, 2u32, 3i32, 4u64, 5i64);
    let (a, .., c) = abcx;

    let a = ABCXenum::A(ABCX(1u32, 2u32, 3i32, 4u64, 5i64));

    match a {
        ABCXenum::A(..) => println!("okey"),
        _ => {},
    }

    match a {
        ABCXenum::A(_) => println!("also okey"),
        _ => {},
    }

    let x = &5;
    match x {
        ref x => println!("it is ref"),
        _ => println!("it is not ref"),
    }

    let x = 1;
    match x {
        1...5 => println!("в диапазое"),
        _ => {},
    }

    let x = ' ';
    match x {
        'а' ... 'и' => println!("ранняя буква"),
        'к' ... 'я' => println!("поздняя буква"),
        _ => println!("что-то ещё"),
    }

    let x = 1;
    match x {
        e @ 1 ... 5 => println!("получили элемент диапазона {}", e),
        _ => println!("что угодно"),
    }

    let mut x: Option<Person> = Some(Person { name: Some("Vlad".to_string()) });
    match x {
        Some(Person { name: a @ Some(_) }) => println!("{:?}", a),
        _ => {}
    }

    let x = 5;
    match x {
        e @ 1 ... 5 | e @ 8 ... 10 => println!("получили элемент диапазона {}", e),
        _ => println!("что угодно"),
    }

    let x = OptionalInt::Value(5);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Получили целое больше пяти!"),
        OptionalInt::Value(..) => println!("Получили целое!"),
        OptionalInt::Missing => println!("Неудача."),
    }

    let x = 5;
    let y = false;
    match x {
        val @ 4 | val @ 5 if !y => println!("{}", val),
        _ => {},
    }
}
