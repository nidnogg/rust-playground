#[derive(Debug)]
struct Drawer {
    used: u32,
    capacity: u32,
}

impl Drawer {
    fn empty(size: u32) -> Self {
        Self {
            used: 0,
            capacity: size,
        }
    }

    fn status(&self) -> String {
        format!("Current capacity: {}/{}", self.used, self.capacity)
    }
}
fn main() {
    let mut drawer = Drawer {
        used: 10,
        capacity: 32,
    };

    println!("Creating drawer with 10 items");
    println!("{}", drawer.status());

    println!("Dropping drawer items");
    drawer.used = 7;
    println!("{}", drawer.status());

    let empty_drawer = Drawer::empty(32);

    println!("{:?}", empty_drawer);
    println!("{}", empty_drawer.status());
    
}
