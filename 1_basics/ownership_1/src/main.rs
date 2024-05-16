fn main() {
    let name = String::from("Morris");
    println!("My name is {name}.");


    greet_user(name);

}


fn greet_user(user: String) {
    println!("Hello {user}!");
}