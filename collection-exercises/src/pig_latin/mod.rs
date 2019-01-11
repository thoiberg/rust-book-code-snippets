pub fn convert_to_pig_latin(sentence: String) -> String {
    let mut pig_latin_words: Vec<String> = Vec::new();

    for word in sentence.split_whitespace() {
        pig_latin_words.push(convert_word_to_pig_latin(String::from(word)));
    }

    let mut pig_latin_sentence = String::new();
    for word in pig_latin_words {
        &pig_latin_sentence.push_str(word.as_str());
        &pig_latin_sentence.push_str(" ");
    }

    pig_latin_sentence
}

fn convert_word_to_pig_latin(word: String) -> String {
    let vowels = ["a", "e", "i", "o", "u"];

    match word.get(0..1) {
        Some(character) => {
            if vowels.contains(&character) {
                convert_word_that_starts_with_a_vowel(&word)
            } else {
                convert_word_that_starts_with_a_consonant(&word)
            }
        },
        None => String::from("dsdsds"),
    }
}

fn convert_word_that_starts_with_a_vowel(word: &String) -> String {
    word.clone() + &String::from("-hay")
}

fn convert_word_that_starts_with_a_consonant(word: &String) -> String {
    let mut characters: Vec<char> = Vec::new();
    let pig_latin_characters = vec!['-', 'a', 'y'];
    for character in word.chars() {
        characters.push(character);
    };

    match characters.split_first() {
        Some((first, rest)) => {
            let mut converted_word: Vec<char> = vec![];
            converted_word.append(&mut rest.to_vec());
            converted_word.push(*first);
            converted_word.append(&mut pig_latin_characters.to_vec());
            converted_word.into_iter().collect()
        }
        None => String::from("dddfdf"),
    }
}
