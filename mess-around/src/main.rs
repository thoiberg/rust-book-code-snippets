fn main() {
    let s = String::from("hello");

    change(s);
}

fn change(some_string: &String) {
    println!("{}", some_string);
}