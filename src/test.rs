//! Примеры теста проекта
//!
//! # Examples
//!
//! ```
//! test()
//! ```

/// long_test() долгий тест, игнорируемый при обычном запуске тестов
///
/// # Examples
///
/// ```
/// #[test]
/// #[ignore]
/// long_test()
/// ```

use diner_philosophers;
use std::thread;

#[test]
fn test() {
    assert_eq!(1, 1);
    assert!(1 == 1);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_panic() {
    assert_eq!(1, 2);
}

#[test]
#[should_panic(expected = "Hello, i am bug!")]
fn test_panic_2() {
    panic!("Hello, i am bug!");
}

#[test]
#[ignore]
fn long_test() {
    thread::spawn(|| {
        thread::sleep_ms(1000);
        diner_philosophers::start();
    }).join().unwrap();
}
