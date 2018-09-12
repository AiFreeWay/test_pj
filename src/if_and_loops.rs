pub fn start() {
    let y = 0;
    let x = if y == 1 { 10 } else if y == 2 { 20 } else { 0 };
    println!("{}", x);
    println!("=====");
    let mut i = 0;
    loop {
        if i == 5 {
            break;
        } else {
            println!("{}", &i);
        }
        i += 1;
    }
    println!("=====");
    let mut j = 0;
    while j != 5 {
        println!("{}", &j);
        j += 1;
    }
    println!("=====");
    for k in 0..5 {
        println!("{}", &k);
    }
    println!("=====");
    for (i, v) in (10..15).enumerate() {
        println!("{} = {}", &i, &v);
    }
    println!("=====");
    let strings = vec!["A", "B", "C"];
    for (i, v) in strings.iter().enumerate() {
        println!("{} = {}", &i, &v);
    }
    println!("=====");
    let mut l = 0;
    while l != 5 {
        l += 1;
        continue;
        println!("{}", &l);
    }
    println!("=====");
    'test_label: for i in 0..10 {
        if i%2 != 0 {
            continue 'test_label;

        }
        println!("{}", &i);
    }
}
