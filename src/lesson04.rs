fn main() {
    // variable to store integer value
    let age = 31;
    println!("Age: {}", age);

    // variable to store floating-point value
    let salary = 342523.23;
    println!("Salary: {}", salary);

    // variable to store string
    let name = "Jackie";
    println!("Name: {}", name);

    // we use the "mut" keyword before the variable name to create a mutable variable
    // declare a mutable variable with value 1
    let mut x = 1;
    println!("Value of x = {}", x);

    // change the value of variable x
    x = 2;
    println!("Updated value of x = {}", x);
}
