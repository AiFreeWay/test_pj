use std::mem;

pub fn start() {
    let x = 5i32;
    let y = x as u64;

    let a = [0u8; 4];
    unsafe {
        let b = mem::transmute::<[u8; 4], u32>(a);
    }
}
