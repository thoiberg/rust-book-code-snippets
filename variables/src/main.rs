fn main() {
    // By default variables are immutable and cannot be reassigned
    // let x = 5;
    // println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);

    // The mut keyword allows the variable to be mutated
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // constants use the const keyword, must have an explicit type and
    // cannot be the result of a function or derived at runtime
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points is {}", MAX_POINTS);

    // shadowing allows some transformations but keeps the variable immutable
    // shadowing allows for type changes while mut does not
    let x = 5;

    let x = x + 1;

    let x = x + 1;

    println!("The value of x is {}", x);

    // Tuples
    // creating
    let tup = (1, "hi", true);

    // destructuring
    let(x, y, z) = tup;

    println!("x: {}, y: {}, z: {}", x, y, z);

    // direct access
    println!("y is: {}", tup.1)

    // Arrays must contain all of the same data type, Tuples don't
    
}
