use crate::Day::Thursday;

// Average enum
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

// Enums kunnen ook waardes bevatten (Iets wat veel talen niet hebben)
#[derive(Debug)]
enum Coordinate {
    TwoD {
        x: usize,
        y: usize,
    },
    ThreeD {
        x: usize,
        y: usize,
        z: usize,
    }
}

fn main() {
    // Assign enum
    let day = Thursday;

    // switch statement/pattern matching
    match day {
        Day::Monday | Day::Tuesday | Day::Wednesday | Day::Friday => {
            println!("Work :(");
        }
        Day::Thursday => {
            println!("TI gilde!");
        }
        Day::Saturday | Day::Sunday => {
            println!("Weekend!");
        }
    }

    // enums met een waarde
    println!("{:?}", Coordinate::ThreeD { x: 0, y: 0, z: 0 })
}
