fn main() {
    println!(
        "================================\nRust Access Control\n================================"
    );

    let is_authenticated: bool = true;
    let is_admin: bool = false;
    let account_active: bool = true;
    let is_admin_endpoint: bool = false;
    let request_count: u32 = 84;
    let risk_score: u8 = 45;

    let is_ip_trusted: bool = true;

    let authenticated: &str = if is_authenticated {
        "PASSED"
    } else {
        "NOT PASSED"
    };

    println!("Authentication: {authenticated}");

    let account_status: &str = if account_active {
        "ACTIVE"
    } else {
        "ACCOUNT_INACTIVE"
    };

    println!("Account Status: {account_status}");

    let role: &str = if is_admin { "ADMIN" } else { "USER" };
    println!("Role: {role}");

    let endpoint: &str = if is_admin_endpoint {
        "ADMIN"
    } else {
        "STANDARD"
    };

    println!("Endpoint: {endpoint}");

    print!("Request Count: ");
    if request_count <= 100 {
        println!("{request_count}");
    } else {
        println!("Maximum requests: 100");
    }

    let risk_level = match risk_score {
        0..=30 => "LOW",
        31..=60 => "MEDIUM",
        61..=70 => "HIGH",
        71..=100 => "CRITICAL",
        _ => "INVALID",
    };
    println!("Risk Score: {risk_score}");
    println!("Risk Classification: {risk_level}");

    let risk_limit: u8 = if is_admin { 85 } else { 70 };

    let access_reason: &str = if !is_authenticated {
        "UNAUTHENTICATED"
    } else if !account_active {
        "ACCOUNT_INACTIVE"
    } else if is_admin_endpoint && !is_admin {
        "INSUFFICIENT_ROLE"
    } else if request_count > 100 {
        "RATE_LIMIT_EXCEEDED"
    } else if risk_score > risk_limit && !is_ip_trusted {
        "RISK_TOO_HIGH"
    } else {
        "GRANTED"
    };

    let access: &str = if access_reason == "GRANTED" {
        "ALLOWED"
    } else {
        "DENIED"
    };

    println!("Access Decision: {access}");
    println!("Reason: {access_reason}");
    println!("================================");
}
