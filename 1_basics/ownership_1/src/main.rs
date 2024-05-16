fn main() {
    // Probeer het voor elkaar


    let name = String::from("Morris");
    println!("My name is {name}.");


    greet_user(name);
    // greet_user(name); // uncomment this line to see the effects
    // probeer nu ervoor te zorgen dat greet_user 2x aangeroepen kan worden


    // Probeer nu door middel van change_name de naam aan te passen (Neem alvast een kijkje bij mutability_2)
    // change_name();
    // greet_user(name);
}


fn greet_user(user: String) {
    println!("Hello {user}!");
}


fn change_name() {
    // try to mutate the name here

}