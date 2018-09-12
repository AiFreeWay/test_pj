use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::Display;

struct Token<'a> {
    raw: Cow<'a, str>
}

impl<'a> Token<'a> {
    pub fn new<S>(raw: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token { raw: raw.into() }
    }
}

struct Containter {
    data: Rc<String>,
}

struct A<'a> {
    val: &'a i32
}

impl<'a> A<'a> {

    fn get(&self) -> &'a i32 {
        self.val
    }
}

pub fn start() {
    let mut i: i32;
    i = 5;
    test_borrow(&i);
    test_borrow(&mut i);

    test_borrow_2(&i);
    test_borrow_2(&mut i);

    // создаем токены.
    let token = Token::new("abc123");
    let token2 = Token::new(String::from("api.example.io"));
    println!("{}, {}", token.raw, token2.raw);

    let mut x = 10;
    inc(&mut x);
    println!("{}", x);

    let mut y = &7;
    let mut a = A { val: &mut x };
    let mut a2 = A { val: a.get() };
    fall(a);
    y = a2.get();
    fall(a2);
    println!("{}", y);

    let data = Rc::new("My data".to_string());

    let container1 = Containter {
        data: data.clone(),
    };

    let container2 = Containter {
        data: data.clone(),
    };

    drop(data);

    println!("{}", container1.data);
    println!("{}", container2.data);

    let x = 8;
    let x_cell = Cell::new(x);
    println!("{}", x_cell.get());
    x_cell.set(10);
    println!("{}", x_cell.get());

    let y = 6;
    let y_ref_cell = RefCell::new(y);
    let ref_borrow = y_ref_cell.borrow();
    println!("{}", *ref_borrow);

    drop(ref_borrow);

    let mut ref_borrow_mut = y_ref_cell.borrow_mut();
    *ref_borrow_mut = 12;
    println!("{}", *ref_borrow_mut);
}

fn test_borrow<T: Borrow<i32> + Display>(a: T)
    where T: Display {
        println!("a borrowed {}", a);
}

fn test_borrow_2(a: &i32) {
    println!("a borrowed {}", a);
}

fn inc(num: &mut i32) {
    *num += 1;
}

fn fall(a: A) {

}
