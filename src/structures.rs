struct XY<'a> {
    x: u32,
    y: &'a mut u32
}

struct XYZ {
    x: u32,
    y: u32,
    z: u32
}

struct MethodBranch;

impl MethodBranch {
    fn a(&self) -> &MethodBranch {
        println!("a");
        &self
    }

    fn b(&self) -> &MethodBranch {
        println!("b");
        &self
    }
}

struct Color(u8, u8, u8);

pub fn start() {
    let xy = XY { x: 5, y: &mut 6 };
    *xy.y = 7;
    println!("{}, {}", xy.x, xy.y);

    let xyz = XYZ { x: 1, y: 2, z: 3};
    let xyz2 = XYZ { z: 4, ..xyz };
    println!("{}, {}, {}", xyz2.x, xyz2.y, xyz2.z);

    let color: Color = Color(1, 2, 3);
    println!("color {}, {}, {}", color.0, color.1, color.2);

    let methodBranch = MethodBranch {};
    methodBranch.a().b();
}
