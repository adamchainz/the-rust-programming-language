use std::collections::HashMap;

fn main() {
    let integers = vec![17, 1004, -22, 38, 9002, 77_777, 38];

    println!("Numbers: {:?}", integers);
    println!("Mean is {}", mean(&integers));
    println!("Median is {}", median(&integers));
    println!("Mode is {}", mode(&integers));
}

fn mean(integers: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    for i in integers.iter() {
        sum += i;
    }
    return sum as f64 / integers.len() as f64;
}

fn median(integers: &Vec<i32>) -> i32 {
    let mut sorted = integers.clone();
    sorted.sort();
    sorted[sorted.len() / 2]
}

fn mode(integers: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for i in integers.iter() {
        counts.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut highest_count = 0;
    let mut modal_value = 0;
    for (value, count) in &counts {
        if count > &highest_count {
            highest_count = *count;
            modal_value = **value;
        }
    }
    return modal_value;
}
