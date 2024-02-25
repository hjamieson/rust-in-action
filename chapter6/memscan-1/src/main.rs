#![allow(dead_code)]

fn main() {
    part3();
}

static GLOBAL: i32 = 0;

fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn part2(){
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    println!("GLOBAL: {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("fn_int: {:p}", fn_int);
}

fn segfaults() {
    let mut n_nonzero = 0;

    for i in 1..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe {*ptr};

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }

        println!("non-zero bytes in memory: {}", n_nonzero);
    }
}

fn part3(){
    #[derive(Debug)]
    struct Foo {
        id: i32,
    }

    let foo = Foo{id: 10,};
    let foo_ptr = &foo as *const Foo;
    let id_ptr = &foo.id as *const i32;
    println!("foo_ptr: {:p}", foo_ptr);
    println!("id_ptr: {:p}", id_ptr);
    unsafe {
        println!("*foo : {:?}", *foo_ptr);
        println!("*id_ptr: {}", *id_ptr);
    }

}