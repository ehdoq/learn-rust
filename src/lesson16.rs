fn main() {
    // an array of numbers
    let numbers = [1, 2, 3, 4, 5];

    // create a slice of 2nd and 3rd element
    let slice = &numbers[1..3];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);

    //--------------------------------------

    // omit the start index
    let slice2 = &numbers[..3];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice2);

    //--------------------------------------

    // omit the end index
    let slice3 = &numbers[2..];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice3);

    //--------------------------------------

    let numbers = [1, 2, 3, 4, 5];

    // omit the start index and the end index
    // reference the whole array
    let slice4 = &numbers[..];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice4);

    //--------------------------------------

    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("array = {:?}", colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("original slice = {:?}", sliced_colors);

    // change the value of the original slice at the first index
    sliced_colors[1] = "purple";

    println!("changed slice = {:?}", sliced_colors);
}
