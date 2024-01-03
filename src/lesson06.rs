fn main() {
    /*
        Integer Data Types
        Size        Signed      Unsigned
        8-bit	    i8	        u8
        16-bit	    i16	        u16
        32-bit	    i32	        u32
        64-bit	    i64	        u64
        128-bit	    i128	    u128
    */

    /*
        Floating Point Type
        Size        Signed
        32-bit	    f32
        64-bit	    f64
    */

    let alphabet: char = 'c';
    let special_character: char = '$';
    let number: i32 = 200;
    let signed_number: i32 = -200;
    let unsigned_number: u32 = 300;
    let float32: f32 = 3.1;
    let float64: f64 = 45.0000031;
    let bool_true: bool = true;
    let bool_false: bool = false;

    println!("alphabet = {alphabet}");
    println!("special_character = {special_character}");
    println!("number = {number}");
    println!("signed_number = {signed_number}");
    println!("unsigned_number = {unsigned_number}");
    println!("float32 = {float32}");
    println!("float64 = {float64}");
    println!("bool_true = {bool_true}");
    println!("bool_false = {bool_false}");
}
