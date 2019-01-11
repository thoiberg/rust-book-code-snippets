enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
   let v: Vec<i32> = Vec::new();
   let v2 = vec![1,2,3];
   let mut v3 = Vec::new();

    // adding to a vector
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // reading from a vector
    let third: &i32 = &v2[2];

    let v4 = vec![1, 2, 3, 4, 5];
    let v4_index = 2;

    match v4.get(v4_index) {
        Some(_) => { println!("Reachable element at index: {}", v4_index); },
        None => { println!("Unreachable element at index: {}", v4_index); },
    }

    // let does_not_exist = &v[100]; // causes the app to panic
    let does_not_exist = v.get(100); // returns a None which can be handled gracefully

    let v5 = vec![100, 32, 57];

    for i in &v5 {
        println!("{}", i);
    }

    // mutating the values in the vector
    let mut v6 = vec![100, 32, 57];

    for i in &mut v6 {
        *i += 50;
    }

    for i in &v6 {
        println!("{}", i);
    }

    // storing multiple data types using enums
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
