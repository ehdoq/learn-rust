fn main() {
    println!("Hello World!");

    //Single comment line

    /*
    Multi
    comment
    line
    */

    print!("Hello ");
    print!("World!");

    let x: i32 = 5;
    println!("x = {}", x);

    let age = 31;
    let name = "Jack";

    println!("Name = {}, Age = {}", name, age);
    println!("Name = {0}, Age = {1}", name, age);

    print!("Rust is fun!\nI love Rust programming.");
}
