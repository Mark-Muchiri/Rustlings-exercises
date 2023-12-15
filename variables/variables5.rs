// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number: &str = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    /*
    Added a let to the second `number variable`
    This is called shadowing. Meaning the second variable with
    the same name as the 1st shadows it.
     */
    let number: i8 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
