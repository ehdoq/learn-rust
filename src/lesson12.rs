fn main() {
    let mut i = 1;

    // start of outer loop
    while i <= 5 {
        let mut j = 1;

        // start of inner loop
        while j <= 5 {
            print!("*");

            // condition to exit the inner loop
            if j == 3 {
                // terminate the inner loop
                break;
            }

            j += 1;
        }

        println!("");

        i += 1;
    }
}
