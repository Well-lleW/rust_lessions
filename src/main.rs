// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]
use rust_lessons::{area_of, volume, ding, on_off, print_difference, print_array, print_distance, double, count, sum};

fn main() {
    // zero
    println!("Hello, world!");
    exercise_a();
    exercise_b();
    exercise_c();
    exercise_d();
}

fn exercise_a() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    let ready:i32 = READY_AMOUNT;
    let missiles :i32 = STARTING_MISSILES;
    println!("Firing {} of my {} missiles", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}

fn exercise_b() {
    let width = 4;
    let height = 7;
    let depth = 10;
    // 1. Try running this code with `cargo run` and take a look at the error.
    //
    // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
    // doing `cargo run` should succeed and print something out.
    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
    //    the code again and make sure it worked (you should get an area of 28).

    // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
    //    Create the `volume` function!  It should:
    //    - Take three arguments of type i32
    //    - Multiply the three arguments together
    //    - Return the result (which should be 280 when you run the program).
    //
    // If you get stuck, remember that this is *very* similar to what `area_of` does.
    //
    println!("Volume is {}", volume(width, height, depth));
}

fn exercise_c() {
    let coords: (f32, f32) = (6.3, 15.0);
    // 1. Pass parts of `coords` to the `print_difference` function. This should show the difference
    // between the two numbers in coords when you do `cargo run`.  Use tuple indexing.
    //
    // The `print_difference` function is defined below the `main` function. It may help if you look
    // at how it is defined.
    //
    print_difference( coords.0, coords.1 );   // Uncomment and finish this line


    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f32; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    let coords_arr= [coords.0, coords.1];             // create an array literal out of parts of `coord` here
    print_array(coords_arr);        // and pass it in here (this line doesn't need to change)


    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    ding(series[series.len()-1]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    //
    on_off(mess.2[1].0);

    // 5.  What a mess -- functions in a binary! Let's get organized!
    //
    // - Make a library file (src/lib.rs)
    // - Move all the functions (except main) into the library
    // - Make all the functions public with `pub`
    // - Bring all the functions into scope using use statements. Remember, the name of the library
    //   is defined in Cargo.toml.  You'll need to know that to `use` it.
    //
    // `cargo run` should produce the same output, only now the code is more organized. 🎉

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
}

fn exercise_d() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // 1a. Your task: handle the command-line arguments!
        //
        // - If arg is "sum", then call the sum() function
        // - If arg is "double", then call the double() function
        // - If arg is anything else, then call the count() function, passing "arg" to it.

        if arg == "sum" {
            sum();
        }else if arg == "double" {
            double();
        }else {
            count(arg);
        }

        // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"
    }
}

