fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn can_be_expressed(n: u32) -> bool {
    for p in 2..n {
        if is_prime(p) {
            let mut k = 1;
            while p + 2 * k * k <= n {
                if p + 2 * k * k == n {
                    return true;
                }
                k += 1;
            }
        }
    }
    false
}



pub fn goldbach_conjecture() -> String {
    let mut count = 0;
    let mut n = 9;
    let mut result = String::new();
    
    while count < 2 {
        if !is_prime(n) && !can_be_expressed(n) {
            if !result.is_empty() {
                result.push_str(",");
            }
            result.push_str(&n.to_string());
            count += 1;
        }
        n += 2; // odd only
    }
    
    result
}
