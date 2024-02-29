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

    let honkers: Vec<Honks> = vec![Car{}, Truck{}, Scooter{}];
    for honker in honkers {
        honker.honk();
    }
}