fn main() {
    let person = Person::new(String::from("Morris"), 23, String::from("[name]@hu.nl"));

    println!("{}", person.describe());
}

struct Person {
    name: String,
    age: u8,
    email: String,
}

impl Person {
    // New is een vaak gebruikte method name in rust om een struct the creëren.
    fn new(name: String, age: u8, email: String) -> Person {
        Person { name, age, email }
    }
}


// Dit is een trait
trait PrettyToString {
    // onderliggende implementatie ligt aan de struct
    fn describe(&self) -> String;

    // een trait kan ook een al uitgewerkte methode bevatten
    fn greet() {
        println!("Hello");
    }
}


// Defineer een trait op een struct
impl PrettyToString for Person {
    fn describe(&self) -> String {
        format!("Name: {}, Age: {}, Email: {}", self.name, self.age, self.email)
    }
}
