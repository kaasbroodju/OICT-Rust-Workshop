fn main() {
    // In rust zijn variable standaard immutable.
    let x = 4;
    x += 1;

    // je moet dan specificeren dat een waarde bewerkbaar is
    let mut i = 5;
    i += 1;
}
