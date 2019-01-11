fn main() {
    another_function(5, 6);

    // variables can be assigned to expressions, as long as expressions return values
    let a = {
        let b = 3;
        b + 1
    };

    println!("The value of a is {}", a);

    println!("The value of five() is {}", five());

    let one_more_than_five = plus_one(five());
    println!("1 above five is {}", one_more_than_five);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    // returning values are not appended with ; 
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}