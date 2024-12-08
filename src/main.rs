use advent_of_code_2024_rust::*;

fn main() {

    let time = std::time::Instant::now();
    let result = day2p2::execute();
    let time = time.elapsed();
    println!("Result: {}", result);
    println!("Execution Time: {} us.", time.as_micros());
}
