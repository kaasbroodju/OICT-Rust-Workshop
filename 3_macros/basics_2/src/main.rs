// defineer op deze manier een macro
macro_rules! greet {
    // defineer hier je inputs van je macro.
    // Zie dit artikel over welke types er allemaal zijn: https://doc.rust-lang.org/rust-by-example/macros/designators.html
    ($name:literal) => {
        // defineer hier de 'body' van de macro
        println!("Hello {}!", $name);
    };
}


fn main() {
    // custom macro usage
    greet!("Morris");
    // greet!("Morris", "Robert"); // Wat moeten we doen als we meerdere mensen willen begroeten?
}
