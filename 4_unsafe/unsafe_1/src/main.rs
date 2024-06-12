fn main() {
    let mut a: i32 = 5;
    let mut b: i32 = 10;

    // Print the original values
    println!("Before swap: a = {}, b = {}", a, b);

    // Use an unsafe block to swap the values using raw pointers
    unsafe {
        // Create raw pointers to the variables
        let a_ptr: *mut i32 = &mut a;
        let b_ptr: *mut i32 = &mut b;

        // Swap the values
        let temp = *a_ptr;
        *a_ptr = *b_ptr;
        *b_ptr = temp;
    }

    // Print the swapped values
    println!("After swap: a = {}, b = {}", a, b);


    let address = 0x012345usize;
    let r = address as *const i32;

    // println!("{}", r);

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
