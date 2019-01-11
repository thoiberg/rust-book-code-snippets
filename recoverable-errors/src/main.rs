use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    // unwrap calls panic! if an error is returned
    // File::open("hello2.txt").unwrap();

    // expect does the same as unwrap but allows a custom error message;
    // File::open("hello2.txt").expect("file does not exist");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello3.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn smarter_read_username_from_file() -> Result<String, io::Error> {
    // ? after a Result is shorthand for setting the Ok value to the variable on the
    // left hand side, or returning the Error
    let mut f = File::open("hello4.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn smarter_read_username_from_file_with_chaining() -> Result<String, io::Error> {
    let mut s = String::new();
    // ? can also be chained together:
    File::open("hello4.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
