static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn main() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();
    
    println!("GLOBAL:\t\t{:p}", &GLOBAL as *const i32);
    println!("local_str:\t{:p}", local_str as *const str);
    println!("local_int:\t{:p}", &local_int as *const i32);
    println!("boxed_int:\t{:p}", Box::into_raw(boxed_int));
    println!("boxed_str:\t{:p}", Box::into_raw(boxed_str));
    println!("fn_int:\t\t{:p}", fn_int);
}
