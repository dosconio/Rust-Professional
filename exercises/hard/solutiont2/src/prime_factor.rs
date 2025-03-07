use rand::Rng;
use std::cmp;
use std::time::Instant;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// 避免溢出的模乘运算
#[inline]
fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
    let mut res = 0;
    let mut a = a % m;
    let mut b = b;
    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % m;
        }
        a = (2 * a) % m;
        b /= 2;
    }
    res
}

fn pollard_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(2..n);
    let mut y = x;
    let c = rng.gen_range(1..n);
    let mut d = 1;
    while d == 1 {
        x = (mod_mul(x, x, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        d = gcd(x.abs_diff(y), n);
        if d == n {
            return pollard_rho(n);
        }
    }
    d
}

fn is_prime(n: u128) -> bool {
    // if n < 2 {
    //     return false;
    // }
    if n <= 3 {
        return true;
    }
    if n & 1 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut n = number;
    if n < 2 {
        return n;
    }
    let mut max_prime = 1;
    let mut num = n;
    //
    while num > 1 {
        if is_prime(num) {
            max_prime = cmp::max(max_prime, num);
            break;
        }
        let factor = pollard_rho(num);
        max_prime = cmp::max(max_prime, find_max_prime_factor(factor));
        num /= factor;
    }
    max_prime
}
