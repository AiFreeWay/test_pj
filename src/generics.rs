trait Reader<'a, T> {
    fn read(&self) -> &'a T;
}

trait Dummy {
    fn read(&self) -> String;

    fn dummy(&self) {
        println!("DUMMMY");
    }
}

trait Setter<'a, T> {
    fn set(&mut self, val: &'a T);
}

struct Value<'a, T: 'a> {
    val: &'a T
}

impl<'a, T> Reader<'a, T> for Value<'a, T> {
    fn read(&self) -> &'a T {
        &self.val
    }
}

impl<'a, T> Setter<'a, T> for Value<'a, T> {
    fn set(&mut self, value: &'a T) {
        self.val = value
    }
}

impl Dummy for i32 {
    fn read(&self) -> String {
        "This is crazy".to_string()
    }
}

struct Container<T> {
    val: T,
}

impl<'a, T: Setter<'a, u32>> Dummy for Container<T> {
    fn read(&self) -> String {
        "This is crazy".to_string()
    }
}


pub fn start() {
    // Err print(5);
    let mut val = Value { val : &13u32 };
    print(&val);
    val.set(&20);
    print2(val);
    println!("{}", 5.read());

    let container = Container { val : Value { val : &13u32 } };
    let container2 = Container{ val : "Hello" };
    container.read();
    container.dummy();
    //Err container2.read();
}

fn print<'a, T>(value: &T) where T : Reader<'a, u32> + Setter<'a, u32> {
    println!("{}", value.read());
}

fn print2<'a, T: Reader<'a, u32> + Setter<'a, u32>>(value: T) {
    println!("{}", value.read());
}
