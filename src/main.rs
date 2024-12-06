use advent_of_code_2024_rust::*;

fn main() {

    let time = std::time::Instant::now();
    day1::execute();
    let time = time.elapsed();
    println!("Execution Time: {} us.", time.as_micros());
}
