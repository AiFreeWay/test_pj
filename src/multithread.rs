use std::thread;
use std::sync::{Mutex, Arc, RwLock};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub fn start() {

    test_rw_lock();

    hello();

    many_thread(2);

    goodbye();
}

fn test_rw_lock() {
    let lock = RwLock::new("From Lock".to_string());
    let lock_arc = Arc::new(lock);

    let lock_arc_clone = lock_arc.clone();
    thread::spawn(move || {

        thread::sleep_ms(1000);
        let mut lock = lock_arc_clone.write().unwrap();
        *lock = "New from lock".to_string();
    });

    let lock_arc_clone2 = lock_arc.clone();
    thread::spawn(move || {
        for i in 0..20 {
            thread::sleep_ms(100);
            let lock = lock_arc_clone2.read().unwrap();
            println!("{} 1", lock);
        }
    });

    let lock_arc_clone3 = lock_arc.clone();
    thread::spawn(move || {
        for i in 0..20 {
            thread::sleep_ms(100);
            let lock = lock_arc_clone3.read().unwrap();
            println!("{} 2", lock);
        }
    });
    thread::sleep_ms(2500);
}

fn cook_burgers(burger_count: usize) {

    let (send_chan, receive_chan) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..burger_count {
            thread::sleep_ms(100);
            send_chan.send(i+1);
        }
        drop(send_chan);
    });

    loop {
        match receive_chan.recv() {
            Ok(res) => { println!("{} burger cooked", res); },
            Err(_e) => { break },
        }
    };
}

fn many_thread(thread_count: usize) {
    let (send_chan, recv_chan) = mpsc::channel();

    for i in 0..thread_count {
        let send_chan = send_chan.clone();

        match thread::spawn(move || {
            send_chan.send(i);
            drop(send_chan);
            panic!("Done");
        }).join() {
            Err(e) => { println!("Panic on {} thread ", i); },
            _ => {},
        };
    }

    for i in 0..thread_count {
        match recv_chan.recv() {
            Ok(res) => { println!("{}", res); },
            Err(e) => {},
        }
    }
}


fn bytes_iter() {
    let (tx, rx) = mpsc::channel::<u32>();
    start_treads(tx);
    for _ in 0..1 {
        match rx.recv() {
              Ok(r) => {
                  println!("result {}", r);
              },
              Err(e) => {
                  println!("error {}", e);
              }
          };
    }
}

fn start_treads(tx: Sender<u32>) {
    let mutex = Arc::new(Mutex::new(vec![1]));

    match thread::spawn(move || -> u32 {
        let mut result = 0;
        for _ in 0..100 {
            let mutex = mutex.clone();

            result = thread::spawn(move || -> u32 {
                let mut mutex_value = mutex.lock().unwrap();
                for i in 0..100 {
                    if i >= mutex_value[0] {
                        mutex_value[0] += i;
                        println!("{}", mutex_value[0]);
                    }
                };
                mutex_value[0]
            }).join().unwrap();
        }
        result
    }).join() {
        Ok(res) => {
            tx.send(res);
        },
        Err(e) => {
            panic!(e);
        },
    }
}

fn hello() {
    thread::spawn(|| {
        println!("Hello");
    }).join();
}

fn goodbye() {
    let handler = thread::spawn(|| {
        "Good by"
    });
    println!("{}", handler.join().unwrap());
}
