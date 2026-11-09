fn main() {
    println!(
        "================================\nRust Score Calculator\n================================"
    );

    let programming: f64 = 85.5;
    let mathematics: f64 = 90.0;
    let networking: f64 = 78.5;

    let total: f64 = programming + mathematics + networking;
    let average: f64 = total / 3.0;
    let remaining: f64 = 300.0 - total;

    let total_int: i32 = total as i32; //254.0 as i32 ได้ 254
    // let average : i32 = average as i32; // 84.666 as i32 ได้ 84 (ตัดทิ้ง ไม่ปัดเศษ)

    println!("Programming: {programming}");
    println!("Mathematics: {mathematics}");
    println!("Networking: {networking}");
    println!("Total Score: {total}");
    println!("Average Score: {:.2}", average);
    println!("Remaining Score: {remaining}");
    println!("Total Score (Integer): {total_int}");

    println!("================================");
}
