fn main() {
    println!(
        "================================\nRust Variables Lab\n================================"
    );

    // ตัวแปร immutable ทั้งหมด (ใช้ let โดยไม่มี mut) พร้อม type annotation
    let developer: &str = "Ray";
    let role: &str = "Backend Developer";
    let level: i32 = 1;
    let score: f64 = 87.5;
    let is_learning: bool = true;
    let language: &str = "Rust";

    println!("Developer: {developer}");
    println!("Role: {role}");
    println!("Level: {level}");
    println!("Score: {score}");
    println!("Learning: {is_learning}");
    println!("Language: {language}");

    let complete: i32 = 2; // ขออนุญาติเล่นเลยละกัน
    println!("Completed Exercises: {complete}");
    println!("================================");
}
