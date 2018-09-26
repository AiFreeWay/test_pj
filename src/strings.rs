pub fn start() {
    let mut text = "Hello".to_string();
    text.push_str(" world");
    println!("{}", text);
    takes_slice(&text);


    let hachiko = "犬";
    for b in hachiko.as_bytes() {
        print!("{} ", b);
    }
    println!("");
    for c in hachiko.chars() {
        print!("{} ", c);
    }
    println!("");

    //Get symbol
    let dog = hachiko.chars().nth(0).unwrap();
    println!("dog {} ", dog);


    // Concat must be first string and next str's
    // String+str+str+str
    let a: String = String::new()+"Hello";
    let b = "man";
    let c = a+" "+b;
    let d = &c[0..5];
    println!("{}", d);
}


fn takes_slice(slice: &str) {
    println!("takes_slice: {}", slice);
}
