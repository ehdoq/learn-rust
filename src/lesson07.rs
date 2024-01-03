fn main() {
    let a = 20;
    let b = 2;

    // add two variables using + operator
    let x = a + b;
    println!("{} + {} = {}", a, b, x);

    // subtract two variables using - operator
    let y = a - b;
    println!("{} - {} = {}", a, b, y);

    // multiply two variables using * operator
    let z = a * b;
    println!("{} * {} = {}", a, b, z);

    // divived two variables using / operator
    let q = a / b;

    println!("{} / {} = {}", a, b, q);

    // arithmetic remainder using % operator
    let c = a % b;

    println!("{} % {} = {}", a, b, c);

    /*
        += (addition assignment)	   =>   a += b	=> a = a + b
        -= (subtraction assignment)	   =>   a -= b	=> a = a - b
        *= (multiplication assignment) =>   a *= b	=> a = a * b
        /= (division assignment)	   =>   a /= b	=> a = a / b
        %= (remainder assignment)	   =>   a %= b	=> a = a % b
    */

    /*
        > (Greater than)	          =>  a > b	    =>  true if a is greater than b
        < (Less than)	              =>  a < b	    =>  true if a is less than b
        >= (Greater than or equal to) =>  a >= b    =>  true if a is greater than or equal to b
        <= (Less than or equal to)	  =>  a <= b    =>  true if a is less than or equal to b
        == (Equal to)	              =>  a == b    =>  true if a is equal to b
        != (Not equal to)	          =>  a != b    =>  true if a is not equal to b
    */

    /*
        && (Logical AND)  =>	exp1 && exp2  =>	returns true if both exp1 and exp2 are true
        || (Logical OR)   =>	exp1 || exp2  =>	returns true if any one of the expressions is true
        ! (Logical NOT)   =>	!exp	      =>    returns true if the expression is false and returns false, if it is true
    */
}
