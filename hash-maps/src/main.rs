use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // returns an Option and needs to be handled with pattern matching
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    // combines two vectors together into a HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // HashMap<_, _> tells Rust to infer types
    let more_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // HashMap takes ownership of field_name and field_value
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);


    // entry and or_insert allows you to only add a value for a key if one does not
    // already exist
    let mut other_scores = HashMap::new();
    other_scores.insert(String::from("Blue"), 10);

    other_scores.entry(String::from("Blue")).or_insert(50);
    other_scores.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", other_scores);


    let text = "Hello world wonderful world";
    let mut word_map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference that can be modified
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_map);
}
