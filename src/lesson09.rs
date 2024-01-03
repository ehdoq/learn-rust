fn main() {
    let mut number = -2;

    // initiate an infinite loop
    loop {
        println!("Loop forever!");

        // stop infinite loop
        break;
    }

    // infinite loop starts here
    loop {
        number += 1;
        println!("{}", number);

        if number >= 10 {
            // exit the loop
            break;
        }
    }
}
