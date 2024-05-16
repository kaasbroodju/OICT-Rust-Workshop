fn main() {
    // In rust zijn variable standaard immutable.
    let x = 4;

    // dus als ik de waarde van x wil veranderen kan dat niet
    // x = 5; // uncomment to see effect

    // je moet dan specificeren dat een waarde bewerkbaar is
    let mut i = 5;
    i += 1;
}
