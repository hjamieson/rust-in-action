struct Car {}
struct Truck {}
struct Scooter {}

trait Honks {
    fn honk(&self) {
        println!("fffffftt!");
    
    } 
}

impl Honks for Car{}
impl Honks for Truck{}
impl Honks for Scooter{}

fn main() {
    let c = Box::new(Car{});
    let t = Box::new(Truck{});
    let s = Box::new(Scooter{});

    let honkers: Vec<Box<dyn Honks>> = vec![c, t, s];
    for honker in honkers {
        honker.honk();
    }
}