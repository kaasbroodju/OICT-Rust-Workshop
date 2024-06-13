fn main() {
    // Creates a pointer to a specific location
    let address = 0x012345usize;
    let r = address as *const i32;

    // uncomment deze lijn, waarom mag ik deze niet bekijken?
    // println!("{}", r);
    // uncomment deze lijn, waarom mag ik deze niet bekijken?
    // println!("{}", *r);

    let mut num = 5;

    // creates a pointer that points to variable num
    let r1 = &num as *const i32;

    // creates a pointer that points to variable num, this variable is also mutable
    let r2 = &mut num as *mut i32;

    // creates a second pointer that points to variable num, this variable is also mutable
    let r3 = &mut num as *mut i32;

    num = 15;

    unsafe {
        // *r1 = 5;
        *r3 = 6;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r3 is: {}", *r3);
        // *r1 = 5;
        *r2 = 3;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r3 is: {}", *r3);
    }
}
