pub fn start() {
    let mut i = (1, 2);
    let mut j = (3, 4);
    j = i;
    let (k, l) = i;

    let (x,) = (0,);
    println!("{}", x);
    let y = i.0;
    println!("{} \n======", y);

    let i = [1, 2, 3, 4];
    prt(&i[..]);
    let i2 = [1; 4];
    prt(&i2[..]);
    let mut i3 = vec![4, 3, 2, 1];
    i3.remove(1);
    i3.remove(1);
    i3.remove(1);
    prt(&i3[0..2]);

    let func: fn(&[i32]) -> () = prt;
}

fn prt(inp: &[i32]) {
    for i in inp {
        println!("{}", i);
    }
    println!("======");
}
