extern crate libc;

extern {
    fn add(a: libc::c_int, b: libc::c_int) -> libc::c_int;
}

fn main() {
    let a = 5;
    let b= 7;

    unsafe {
        let result = add(a, b);
        println!("The sum of {} and {} is {}", a, b, result);
    }
}