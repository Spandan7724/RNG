use std::collections::HashMap;
use std::io;
use std::time::Instant;
use rng_tester::get_random_u32;

fn get_test_numbers(count: usize) -> io::Result<Vec<u32>> {
    let mut numbers = Vec::with_capacity(count);
    for _ in 0..count {
        numbers.push(get_random_u32()?);
    }
    Ok(numbers)
}

fn main() -> io::Result<()> {
    println!("Running Random Number Generator Tests...\n");
    
    let sample_size = 100_000;
    
    let start_time = Instant::now();
    println!("Generating {} random numbers...", sample_size);
    let numbers = get_test_numbers(sample_size)?;
    println!("Generation time: {:?}\n", start_time.elapsed());

    run_distribution_tests(&numbers);
    run_bit_pattern_analysis(&numbers);
    run_speed_test()?;
    run_entropy_test(&numbers);
    run_sequence_tests(&numbers);

    if check_randomness_criteria(&numbers) {
        println!("\n✅ All randomness criteria passed!");
    } else {
        println!("\n❌ Some randomness criteria failed!");
    }

    Ok(())
}

fn run_distribution_tests(numbers: &[u32]) {
    println!("=== Distribution Tests ===");

    let mean = numbers.iter().map(|&x| x as f64).sum::<f64>() / numbers.len() as f64;
    let expected_mean = (u32::MAX as f64) / 2.0;
    
    let variance = numbers.iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / numbers.len() as f64;
    
    let std_dev = variance.sqrt();

    println!("Mean: {:.2} (Expected: {:.2})", mean, expected_mean);
    println!("Standard Deviation: {:.2}", std_dev);
    
    // Distribution across ranges
    let mut ranges = vec![0; 10];
    let range_size = (u32::MAX as f64) / 10.0;
    
    for &num in numbers {
        let index = (num as f64 / range_size) as usize;
        if index < 10 {
            ranges[index] += 1;
        }
    }

    println!("\nDistribution across ranges:");
    for (i, count) in ranges.iter().enumerate() {
        let percentage = (*count as f64 / numbers.len() as f64) * 100.0;
        println!("Range {}: {:.2}% (Expected: 10.00%)", i, percentage);
    }
    println!();
}

fn run_bit_pattern_analysis(numbers: &[u32]) {
    println!("=== Bit Pattern Analysis ===");

    let mut bit_counts = vec![0; 32];
    for &num in numbers {
        for bit in 0..32 {
            if (num & (1 << bit)) != 0 {
                bit_counts[bit] += 1;
            }
        }
    }

    println!("Bit distribution (should be close to 50% for each bit):");
    for (bit, &count) in bit_counts.iter().enumerate() {
        let percentage = (count as f64 / numbers.len() as f64) * 100.0;
        println!("Bit {}: {:.2}%", bit, percentage);
    }
    println!();
}

fn run_speed_test() -> io::Result<()> {
    println!("=== Speed Test ===");
    
    let iterations = 10_000;
    let start_time = Instant::now();
    
    for _ in 0..iterations {
        get_random_u32()?;
    }
    
    let elapsed = start_time.elapsed();
    let numbers_per_second = iterations as f64 / elapsed.as_secs_f64();
    
    println!("Generated {} numbers in {:?}", iterations, elapsed);
    println!("Speed: {:.2} numbers/second\n", numbers_per_second);
    
    Ok(())
}

fn run_entropy_test(numbers: &[u32]) {
    println!("=== Entropy Analysis ===");
 
    let mut value_counts: HashMap<u32, usize> = HashMap::new();
    for &num in numbers {
        *value_counts.entry(num).or_insert(0) += 1;
    }
    
    let total = numbers.len() as f64;
    let entropy: f64 = value_counts.values()
        .map(|&count| {
            let probability = count as f64 / total;
            -probability * probability.log2()
        })
        .sum();
    
    println!("Empirical entropy: {:.2} bits", entropy);
    println!("Maximum possible entropy for u32: 32 bits");
    println!("Entropy ratio: {:.2}%\n", (entropy / 32.0) * 100.0);
}

fn run_sequence_tests(numbers: &[u32]) {
    println!("=== Sequence Analysis ===");
    

    let mut sum_diff = 0.0;
    let mut sum_diff_squared = 0.0;
    let len = numbers.len() - 1;
    
    for i in 0..len {
        let diff = numbers[i + 1] as f64 - numbers[i] as f64;
        sum_diff += diff;
        sum_diff_squared += diff * diff;
    }
    
    let mean_diff = sum_diff / len as f64;
    let variance_diff = (sum_diff_squared / len as f64) - (mean_diff * mean_diff);
    
    println!("Sequential difference analysis:");
    println!("Mean difference between consecutive numbers: {:.2}", mean_diff);
    println!("Variance of differences: {:.2}", variance_diff);
    

    let mut repeats = 0;
    for i in 1..numbers.len() {
        if numbers[i] == numbers[i-1] {
            repeats += 1;
        }
    }
    
    let repeat_percentage = (repeats as f64 / numbers.len() as f64) * 100.0;
    println!("Repeated numbers: {:.4}% (should be very close to 0%)\n", repeat_percentage);
}

fn check_randomness_criteria(numbers: &[u32]) -> bool {
    let total = numbers.len() as f64;
    

    let mut bit_counts = vec![0; 32];
    for &num in numbers {
        for bit in 0..32 {
            if (num & (1 << bit)) != 0 {
                bit_counts[bit] += 1;
            }
        }
    }
    
    for &count in &bit_counts {
        let percentage = (count as f64 / total) * 100.0;
        if percentage < 48.0 || percentage > 52.0 {
            return false;
        }
    }
    
    let mut repeats = 0;
    for i in 1..numbers.len() {
        if numbers[i] == numbers[i-1] {
            repeats += 1;
        }
    }
    
    let repeat_percentage = (repeats as f64 / total) * 100.0;
    if repeat_percentage > 0.1 {
        return false;
    }

    
    true
}