use std::thread;

pub mod diner_philosophers;
pub mod guessing_game;

#[no_mangle]
pub extern "C" fn start() {
    let handlers: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                compute();
            })
        })
        .collect();

    for h in handlers {
        h.join().unwrap();
    }
}

fn compute() {
    let mut _x: u64 = 0;
    for i in 0..5_000_000 {
        _x += i;
    }
}

#[cfg(test)]
mod test;

#[cfg(any(unix))]
#[cfg(feature = "foo")]
#[cfg_attr(any(unix), cfg(feature = "bar"))]
pub mod cfg;

/// Это пример тестовой документации
///
/// # Examples
///
/// Пример проверки кода документации
/// ```
/// # let x = 5;
/// # let y = 6;
/// println!("{}", x + y);
/// ```
pub mod doc;
pub mod iterators;
pub mod multithread;
pub mod borrow;
pub mod errors;
pub mod ffi;
pub mod simple_types;
pub mod if_and_loops;
