fn main() {
    println!("============================================================");
    println!("\t\tSYSTEM METRICS ANALYZER");
    println!("============================================================");

    let cpu_usage: [f64; 20] = [
        12.5, 18.0, 22.5, 30.0, 35.5, 42.0, 48.5, 55.0, 62.0, 78.5, 85.0, 91.5, 88.0, 76.5, 64.0,
        52.0, 40.5, 32.0, 25.0, 18.5,
    ];

    let memory_usage: [u32; 20] = [
        1200, 1250, 1300, 1350, 1400, 1500, 1600, 1700, 1800, 2000, 2200, 2400, 2300, 2100, 1900,
        1700, 1600, 1500, 1400, 1300,
    ];

    let count_cpu: usize = cpu_usage.len();
    let threshold: f64 = 80.0;
    let high_load_threshold: f64 = 75.0;
    let window_size: usize = 5;

    println!("[1] SYSTEM OVERVIEW \n------------------------------------------------------------");
    println!("{:<22} : {}", "Total Samples", count_cpu);
    println!("{:<22} : {:.2}%", "CPU Threshold", threshold);
    println!(
        "{:<22} : {:.2}%",
        "High Load Threshold", high_load_threshold
    );
    println!("{:<22} : {} samples", "Moving Average Window", window_size);
    println!();

    // Part A — Statistical Analysis
    // คำนวณ Statistics ของ CPU และ Memory โดยใช้ Loop
    // - Total
    // - Average
    // - Minimum พร้อม Index
    // - Maximum พร้อม Index
    // - Standard Deviation ของ CPU
    // ใช้ f64 สำหรับ CPU calculations และแปลง Memory เป็น f64 ตอนคำนวณค่าเฉลี่ย

    let mut total_cpu: f64 = 0.0;
    let mut min_cpu: f64 = cpu_usage[0];
    let mut max_cpu: f64 = cpu_usage[0];

    let mut min_cpu_index: usize = 0;
    let mut max_cpu_index: usize = 0;

    for i in 0..count_cpu {
        total_cpu += cpu_usage[i];

        if cpu_usage[i] < min_cpu {
            min_cpu = cpu_usage[i];
            min_cpu_index = i;
        }

        if cpu_usage[i] > max_cpu {
            max_cpu = cpu_usage[i];
            max_cpu_index = i;
        }
    }

    let average_cpu: f64 = total_cpu / count_cpu as f64;

    let mut sum_squared_diff: f64 = 0.0;

    for i in 0..count_cpu {
        sum_squared_diff += (cpu_usage[i] - average_cpu).powi(2);
    }

    let variance_cpu: f64 = sum_squared_diff / count_cpu as f64;
    let std_dev_cpu: f64 = variance_cpu.sqrt();

    println!("[2] CPU STATISTICS\n------------------------------------------------------------");
    println!("{:<22} : {:.2}%", "CPU Total", total_cpu);
    println!("{:<22} : {:.2}%", "CPU Average", average_cpu);
    println!(
        "{:<22} : {:.2}% (Index {})",
        "CPU Minimum", min_cpu, min_cpu_index
    );
    println!(
        "{:<22} : {:.2}% (Index {})",
        "CPU Maximum", max_cpu, max_cpu_index
    );
    println!("{:<22} : {:.2}%", "CPU Standard Deviation", std_dev_cpu);
    println!();

    let mut total_memory: u64 = 0;
    let mut min_memory: u32 = memory_usage[0];
    let mut max_memory: u32 = memory_usage[0];

    let mut min_memory_index: usize = 0;
    let mut max_memory_index: usize = 0;

    let count_memory: usize = memory_usage.len();

    for i in 0..count_memory {
        total_memory += memory_usage[i] as u64;

        if memory_usage[i] < min_memory {
            min_memory = memory_usage[i];
            min_memory_index = i;
        }

        if memory_usage[i] > max_memory {
            max_memory = memory_usage[i];
            max_memory_index = i;
        }
    }

    let average_memory: f64 = total_memory as f64 / count_memory as f64;

    println!("[3] MEMORY STATISTICS\n------------------------------------------------------------");
    println!("{:<22} : {} MB", "Memory Total", total_memory);
    println!("{:<22} : {:.2} MB", "Memory Average ", average_memory);

    println!(
        "{:<22} : {} MB (Index {})",
        "Memory Minimum", min_memory, min_memory_index
    );

    println!(
        "{:<22} : {} MB (Index {})",
        "Memory Maximum  ", max_memory, max_memory_index
    );
    println!();

    //     Part B — Moving Average
    // คำนวณ CPU Average ของทุก Window ที่มี 5 Samples
    // - Window 0: Index 0–4
    // - Window 1: Index 1–5
    // - Window 2: Index 2–6
    // - ...
    // - Window 15: Index 15–19
    // ต้องหาค่าเฉลี่ยของทุก Window พร้อมหา Window ที่มี Average สูงสุดและต่ำสุด

    let window_count: usize = count_cpu - window_size + 1;
    let mut highest_window_average: f64 = 0.0;
    let mut lowest_window_average: f64 = f64::MAX;
    let mut highest_window_index: usize = 0;
    let mut lowest_window_index: usize = 0;

    println!("[4] MOVING AVERAGE\n------------------------------------------------------------");
    println!("{:<8} {:<12} {:>12}", "Window", "Start-End", "Average CPU");
    println!("------------------------------------------------------------");

    for window_index in 0..window_count {
        let start_index: usize = window_index;
        let end_index: usize = start_index + window_size - 1;
        let mut window_total: f64 = 0.0;

        for j in start_index..=end_index {
            window_total += cpu_usage[j];
        }

        let window_average: f64 = window_total / window_size as f64;

        if window_average > highest_window_average {
            highest_window_average = window_average;
            highest_window_index = window_index;
        }

        if window_average < lowest_window_average {
            lowest_window_average = window_average;
            lowest_window_index = window_index;
        }

        println!(
            "{:<8} {:<12} {:>11.2}%",
            window_index,
            format!("{start_index}-{end_index}"),
            window_average
        );
    }
    println!();
    println!(
        "{:<22} : {:.2}% (Index {})",
        "Highest Window Average", highest_window_average, highest_window_index
    );
    println!(
        "{:<22} : {:.2}% (Index {})",
        "Lowest Window Average", lowest_window_average, lowest_window_index
    );
    println!();

    //     Part C — Anomaly Detection
    // ตรวจ CPU ที่มีค่ามากกว่า 80.0
    // - แสดง Index
    // - แสดง CPU Value
    // - ระบุ NORMAL หรือ ANOMALY
    // - นับจำนวน Anomalies
    // - คำนวณ Anomaly Percentage

    let mut anomaly_count: usize = 0;

    println!("------------------------------------------------------------");
    println!("{:<8} {:<12} {:>12}", "Index", "CPU Usage", "Status");
    println!("------------------------------------------------------------");

    for i in 0..count_cpu {
        let is_anomaly: bool = cpu_usage[i] > threshold;

        let check_anomaly: &str = if is_anomaly { "ANOMALY" } else { "NORMAL" };

        if is_anomaly {
            anomaly_count += 1;
        }

        println!("{:<8} {:<12.2} {:>12}", i, cpu_usage[i], check_anomaly);
    }

    let anomaly_percentage: f64 = anomaly_count as f64 / count_cpu as f64 * 100.0;
    println!();
    println!("{:<22} : {}", "Total Anomalies", anomaly_count);
    println!("{:<22} : {:.2}%", "Anomaly Percentage ", anomaly_percentage);
    println!();

    //     Part D — Consecutive High Load
    // ค้นหาช่วงที่ CPU มากกว่า 75.0 ติดต่อกัน

    // ต้องแสดง:
    // - Start Index
    // - End Index
    // - Duration
    // - Peak CPU
    // - จำนวน High Load Periods

    let mut high_load_period_count: usize = 0;
    let mut i: usize = 0;

    println!("------------------------------------------------------------");
    println!(
        "{:<8} {:<12} {:>10} {:>12}",
        "Period", "Start-End", "Duration", "Peak CPU"
    );
    println!("------------------------------------------------------------");

    while i < count_cpu {
        if cpu_usage[i] > high_load_threshold {
            let start_index: usize = i;
            let mut end_index: usize = i;
            let mut duration: usize = 0;
            let mut peak_cpu: f64 = cpu_usage[i];
            high_load_period_count += 1;

            while i < count_cpu && cpu_usage[i] > high_load_threshold {
                duration += 1;

                if cpu_usage[i] > peak_cpu {
                    peak_cpu = cpu_usage[i];
                }

                end_index = i;
                i += 1;
            }

            println!(
                "{:<8} {:<12} {:>10} {:>11.2}%",
                high_load_period_count,
                format!("{start_index}-{end_index}"),
                duration,
                peak_cpu
            );
        } else {
            i += 1;
        }
    }

    println!();
    println!(
        "{:<22}: {}",
        "Total High Load Periods:", high_load_period_count
    );
    println!();

    //     Part E — CPU Ranking
    // เรียง CPU จากมากไปน้อย พร้อมเก็บ Index ต้นฉบับ

    let mut sorted_cpu: [f64; 20] = cpu_usage;
    let mut sorted_indices: [usize; 20] = [0; 20];
    let mut max_index: usize;
    let mut temp_cpu: f64;
    let mut temp_index: usize;

    for i in 0..count_cpu {
        sorted_indices[i] = i;
    }

    for i in 0..count_cpu - 1 {
        max_index = i;

        for j in (i + 1)..count_cpu {
            if sorted_cpu[j] > sorted_cpu[max_index] {
                max_index = j;
            }
        }

        if max_index != i {
            temp_cpu = sorted_cpu[i];
            sorted_cpu[i] = sorted_cpu[max_index];
            sorted_cpu[max_index] = temp_cpu;

            temp_index = sorted_indices[i];
            sorted_indices[i] = sorted_indices[max_index];
            sorted_indices[max_index] = temp_index;
        }
    }

    println!("------------------------------------------------------------");
    println!("{:<8} {:<12} {:>12}", "Rank", "Index", "CPU Usage");
    println!("------------------------------------------------------------");

    for i in 0..count_cpu {
        let rank: usize = i + 1;
        println!(
            "{:<8} {:<12} {:>12.2}",
            rank, sorted_indices[i], sorted_cpu[i]
        );
    }

    println!("============================================================");
    println!("\t\t ANALYSIS COMPLETE");
    println!("============================================================");
}
