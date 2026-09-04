mod solution;
mod unit_tests;
use memory_stats::memory_stats;
use solution::Solution as sol;
use std::{i32, time::Instant, vec};

fn main() {
    println!("=== Benchmarking ===");

    // --- Small Input Test ---
    println!("[Small Input Test]");
    let s: Vec<i32> = vec![-1,0,1,2,-1,-4];

    let usage_before = memory_stats().unwrap();
    let start_time = Instant::now();

    let _ = sol::three_sum(s);
    
    let duration = start_time.elapsed();
    let usage_after = memory_stats().unwrap();

    let mem_used = usage_after
        .physical_mem
        .saturating_sub(usage_before.physical_mem);

    println!("Execution Time: {:?}", duration);
    println!("Memory Delta:   {} bytes", mem_used);
    println!("Current Memory: {} bytes", usage_after.physical_mem);

    // --- Stress Test (Large Input) ---
    println!("\n[Stress Test - 1000]");

    let s: Vec<i32> = vec![-1; 1000];

    let usage_before_stress = memory_stats().unwrap();
    let start_time_stress = Instant::now();

    let _ = sol::three_sum(s);

    let duration_stress = start_time_stress.elapsed();
    let usage_after_stress = memory_stats().unwrap();

    let mem_used_stress = usage_after_stress
        .physical_mem
        .saturating_sub(usage_before_stress.physical_mem);

    println!("Execution Time: {:?}", duration_stress);
    println!("Memory Delta:   {} bytes", mem_used_stress);
    println!("Current Memory: {} bytes", usage_after_stress.physical_mem);
}   