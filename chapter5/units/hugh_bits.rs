fn main() -> () {
    let a1 = 10u8;
    println!("{:8b}", a1);
    println!("{:8b}", -1i8);
    println!("{:8b}", -2i8);
    
    let f1:f32 = 0.000001;
    let f1b: u32 = unsafe {
        std::mem::transmute(f1)
    };
    println!("{:b}", f1b);
    println!("{:032b}", 0u32);


}