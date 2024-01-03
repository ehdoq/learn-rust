fn main() {
    let mut i = 1;

    // start of outer loop
    while i <= 5 {
        let mut j = 1;

        // start of inner loop
        while j <= 5 {
            j += 1;

            // condition to skip iteration of the inner loop
            if j == 3 {
                // move to the next iteration of the inner loop
                continue;
            }

            print!("*");
        }

        println!("");

        i += 1;
    }
}
