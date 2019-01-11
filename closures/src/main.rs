use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// previous function definition before it was converted into a closure
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T,U,V>
    where T: Fn(U) -> V,
          U: std::cmp::Eq,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T,U,V> Cacher<T,U,V>
    where T: Fn(U) -> V,
          U: Copy,
          U: std::cmp::Eq,
          U: std::hash::Hash,
          V: Copy,
{
    fn new(calculation: T) -> Cacher<T,U,V> {
        let values: HashMap<U,V> = HashMap::new();
        Cacher {
            calculation,
            values,
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today do {} pushups!",
            expensive_closure.value(intensity),
        );
        println!(
            "Next do {} situps!",
            expensive_closure.value(intensity),
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today run for {} minutes",
                expensive_closure.value(intensity),
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 26;
    let simulated_random_number = 4;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn works_with_other_types() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(-3);

        assert_eq!(v1, -3);
    }
}
