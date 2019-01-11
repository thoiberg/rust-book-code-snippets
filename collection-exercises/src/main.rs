// exercises from https://doc.rust-lang.org/book/2018-edition/ch08-03-hash-maps.html

pub mod stats;
pub mod pig_latin;

fn main() {
    let numbers = vec![5, 4, 1, 2, 3, 3];
    stats::stats(numbers);

    let sentence = String::from("apple banana pie");
    let pig_latin_sentence = pig_latin::convert_to_pig_latin(sentence);
    println!("{}", pig_latin_sentence);

}
