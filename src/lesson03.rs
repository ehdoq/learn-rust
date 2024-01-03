fn main() {
    let age = 31;
    let name = "Jack";

    // print the variable using println!
    println!("{}", age);

    // print the variable using print!
    print!("{}", age);

    // print the variables using println!
    println!("Name = {}, Age = {}", name, age);

    // print the variable using println!
    println!("Name = {0}, Age = {1}", name, age);

    // print the variables using println!
    println!("Name = {name}, Age = {age}");

    print!("Rust is fun!\nI love Rust programming.");
}
