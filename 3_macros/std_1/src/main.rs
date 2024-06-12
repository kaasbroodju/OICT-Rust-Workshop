fn main() {
    // print macro. Een wrapper voor alle code om iets te printen naar console.
    println!("Hello, world!");
    println!("{:?}", Coordinate { lat: 164, long: -20 });


    // De vec macro. Een handige wrapper ipv om voor elke waarde .push() aan te roepen.
    let x = vec![1, 2, 3, 4];
    println!("{:?}", x);
}


// Derive macro. De debug voegt code toe zodat je, je struct kan printen
#[derive(Debug)] // Comment deze regel om het effect te zien
struct Coordinate {
    lat: i64,
    long: i64,
}
