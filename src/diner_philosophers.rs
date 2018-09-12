use std::thread;
use std::sync::{Mutex, Arc};

struct Philosoper {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosoper {

    fn new(name: &str, left: usize, right: usize) -> Philosoper {
        Philosoper {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} начала есть.", self.name);
        thread::sleep_ms(1000);
        println!("{} закончила есть.", self.name);
    }
}

pub fn start() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosopers = vec![
        Philosoper::new("Джудит Батлер", 0, 1),
        Philosoper::new("Рая Дунаевская", 1, 2),
        Philosoper::new("Зарубина Наталья", 2, 3),
        Philosoper::new("Эмма Гольдман", 3, 4),
        Philosoper::new("Анна Шмидт", 0, 4),
     ];

     let handlers:Vec<_> = philosopers.into_iter().map(|p| {
         let table = table.clone();

         thread::spawn(move || {
             p.eat(&table);
         })
     }).collect();

     for handler in handlers {
         handler.join().unwrap();
     }
}
