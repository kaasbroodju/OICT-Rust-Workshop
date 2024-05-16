fn main() {
    let numbers = vec![10, 20, 30, 40, 50];


    let probably_null = numbers.get(100);

    match probably_null {
        None => {
            println!("no value");
        }
        Some(number) => {
            println!("{}", number);
        }
    }
}
