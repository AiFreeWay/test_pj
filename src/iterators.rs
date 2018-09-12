use std::iter::Iterator;

struct MyIter {
    size: usize,
    position: usize,
    items: Vec<String>,
}

impl MyIter {

    fn new() -> MyIter {
        MyIter {
            size: 3,
            position: 0,
            items: vec!["A".to_string(), "B".to_string(), "C".to_string()],
        }
    }
}

impl Iterator for MyIter {

    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.size {
            self.position = 0;
            None
        } else {
            let item = &self.items[self.position];
            self.position += 1;
            Some(item.to_string())
        }
    }
}

pub fn start() {
    let mut range = MyIter::new();

    loop {
        match range.next() {
            Some(v) => { println!("{}", v); },
            None => { break },
        }
    }

    println!("");
    let numbers = vec![0, 1, 2, 3, 4];
    for i in numbers {
        println!("{}", i);
    }

    println!("");
    let greatly_42 = (1..100).find(|x| *x > 42);
    match greatly_42 {
        Some(r) => { println!("{}", r); },
        None => {},
    };

    println!("");
    let sum = (1..5).fold(0, |sum, x| sum+x);
    println!("{}", sum);

    println!("");
    let vec_0_100 = (1..100).collect::<Vec<u8>>();
    let mut vec_iterator = vec_0_100.iter();
    let remainder = vec_iterator.find(|x| *x%16 == 0);

    match remainder {
        Some(r) => {
            println!("{}", r);
        },
        None => {},
    };

    println!("");
    for i in (0..5).map(|x| x*2) {
        println!("{}", i);
    }

    println!("");
    for i in (0..100).take(5) {
        println!("{}", i);
    }

    println!("");
    for i in (0..100).filter(|x| x%8 == 0).filter(|x| x%32 == 0) {
        println!("{}", i);
    }
}
