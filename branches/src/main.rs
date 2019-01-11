fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if expressions must evaluate to the same type
    let x = if true {
        5
    } else {
        6
    };

    println!("The value of x is {}", x);

    // loops infinitely
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // break with a value will return the value from the loop
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // for loops
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);

        index = index + 1;
    }

    // iterating through array
    let b = [60,70,80,90,100];

    for element in b.iter() {
        println!("the value is {}", element);
    }
}
