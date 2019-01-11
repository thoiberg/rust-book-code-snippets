use std::collections::HashMap;

pub fn stats(numbers: Vec<usize>) {
    let mean = mean(&numbers);
    let median = median(&numbers);
    let mode = mode(&numbers);

    println!("mean is {}", mean);
    assert_eq!(mean, 3);

    println!("median is {}", median);
    assert_eq!(median, 3);

    println!("mode is {}", mode);
    assert_eq!(mode, 3);
}

fn mean(numbers: &Vec<usize>) -> usize {
   let mut total = 0;
    for i in numbers {
        total = total + i;
    }

    total / numbers.len()
}

fn median(numbers: &Vec<usize>) -> usize {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();
    let median_index = sorted_numbers.len() / 2;

    return sorted_numbers[median_index];
}

fn mode(numbers: &Vec<usize>) -> usize {
    let mut occurrences_count: HashMap<usize, usize> = HashMap::new();

    for i in numbers {
        let count = occurrences_count.entry(*i).or_insert(0);
        *count += 1;
    }

    // find the largest number and return
    let mut most_frequent_number = 0;
    let mut highest_value = 0;
    for (key, value) in &occurrences_count {
        if value > &highest_value {
            highest_value = *value;
            most_frequent_number = *key;
            // most_frequent_number = &key;
        }
    }

    most_frequent_number
}
