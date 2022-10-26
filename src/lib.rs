//exercise_b
pub fn volume(x: i32, y: i32, z: i32) -> i32 {
    let volume = x * y * z;
    volume
}

pub fn area_of(x: i32, y: i32) -> i32 {
    // 2a. Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y by multiplying x and y and returning the result.
    let area = x * y;
    area
    // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
    //            `return` on the last line of a function. Change the last line to be a
    //            "tail expression" that returns a value without using `return`.
    //            Hint: `cargo clippy` will warn you about this exact thing.
}

//exercise_c
pub fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

pub fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

pub fn print_distance((x, y): (f32, f32)) {
    // Using z.0 and z.1 is not nearly as nice as using x and y.  Lucky for
    // us, Rust supports destructuring function arguments.  Try replacing "z" in
    // the parameter list above with "(x, y)" and then adjust the function
    // body to use x and y.
    println!(
        "Distance to the origin is {}",
        ( x.powf(2.0) + y.powf(2.0) ).sqrt());
}

//exercise_d
pub fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (increment the `sum` variable).  Hint: You should get 255
    // Run it with `cargo run sum`
 for i in 7..=23{
     sum += i;
 }

    println!("The sum is {}", sum);
}

pub fn double() {
    let mut count = 0;
    let mut x = 1;
    // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
    // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
    // with `cargo run double`  Hint: The answer is 9 times.

    while x < 500 {
        x *= 2;
        count += 1;
    }
    println!("You can double x {} times until x is larger than 500", count);
}

pub fn count(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    let mut count = 0;
    loop {
        count += 1;
        print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
        println!(); // This will output just a newline at the end for cleanliness.
        if count >= 8 {break};
    }

}
