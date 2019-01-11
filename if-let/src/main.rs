fn main() {
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three!"),
        _ => (),
    }

    // equivalent to
    if let Some(3) = some_u8_value {
        println!("three!");
    }
    // but loses the exhaustive checking of the match statement
}
