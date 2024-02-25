#![allow(unused, dead_code)]
use std::{ffi::CStr, mem::size_of};

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    // fun1();
    fun2();
}

fn fun1() {
    let a: usize = 42;

    let b: &[u8; 10] = &B;

    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("Location: {:p}", &a);
    println!("size: {:?} bytes", size_of::<usize>());
    println!("value: {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!("Location: {:p}", &b);
    println!("size: {:?} bytes", size_of::<[u8; 10]>());
    println!("points to: {:?}", b);
    println!();

    println!("c (a \"box\" for C):");
    println!("Location: {:p}", &c);
    println!("size: {:?} bytes", size_of::<Box<[u8]>>());
    println!("points to: {:p}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!("Location: {:p}", &B);
    println!("size: {:?} bytes", size_of::<[u8; 10]>());
    println!("value: {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!("Location: {:p}", &C);
    println!("size: {:?} bytes", size_of::<[u8; 11]>());
    println!("value: {:?}", C);
    println!();
}

fn fun2() {
    use std::borrow::Cow;
    use std::os::raw::c_char;

    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
        println!("a: {}, b: {}, c: {}", a, b, c);
    }
}

fn is_strong<T: AsRef<str>>(password: T) -> bool{
    password.as_ref().len() > 6
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::transmute;


    #[test]
    fn pointer1() {
        let a: i64 = 42;
        let a_ptr = &a as *const i64;
        println!("ptr a address:{:p}", a_ptr);

        let b: [u8; 2] = [0x01, 0x02];
        let b0 = &b[0] as *const u8;
        let b1 = &b[1] as *const u8;
        println!("b0: {:p}", b0);
        println!("b1: {:p}", b1);
        unsafe {
            println!("value of b0: {}", *b0);
            println!("value of b1: {}", *b1);
        }

        let c: &mut [u8; 1] = &mut [10; 1];
        c[0] = 0xf;
        println!("c[0]: {}", c[0]);
        let c_ptr = c as *mut u8;
        unsafe {
            println!("c_ptr: {:p}, value: {}", c_ptr, *c_ptr);
        }
    }

    #[test]
    fn pointer2() {
        unsafe {
            let a: i64 = 255;
            let ptr_a = &a as *const i64;
            println!("ptr_a value = {}", *ptr_a);
        }
    }

    #[test]
    fn pointer3(){
        let a: i64 = 42;
        let a_ptr = &a as *const i64;
        let a_addr: usize = unsafe {
            transmute(a_ptr)
        };
        println!("a: {}, ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
    }

    #[test]
    fn pointer4(){
        let ptr = 42 as *const Vec<String>;
        unsafe {
            let new_addr = ptr.offset(4);
            println!("{:p} -> {:p}", ptr, new_addr);
        }
    }

    #[test]
    fn pointer_is_strong(){
        let good_pw = "Hello55".to_string();
        let bad_pw = "123456";
        assert!(is_strong(good_pw));
        assert!(!is_strong(bad_pw));
        //println!("{}", good_pw);    good_pw got moved, so you can't do this!
        println!("{}", bad_pw); // this is valid because bad_pw was borrowed!
    }

    #[test]
    fn pointer_heap_1(){
        let a = 40;
        let b = Box::new(60i32);
        let result = a + *b;
        assert!(result == 100);
    }

    #[test]
    fn pointer_heap_2(){
        let a = Box::new(1);
        let b = Box::new(1);
        let c = Box::new(1);
        let result = *a + *b + *c;

        drop(a);
        let d = Box::new(1);
        let result2 = *b + *c + *d;

        println!("{} {}", result, result2);
        assert_eq!(result, result2);
    }
}
