trait RoundToFour {
    fn round_to(self, places: u32) -> f64;
}

impl RoundToFour for f64 {
    fn round_to(self, places: u32) -> f64 {
        let factor = 10f64.powi(places as i32);
        (self * factor).round() / factor
    }
}

pub fn new_birthday_probability(n: u32) -> f64 {
    if n < 2 {
        return 0.0;
    }
    let mut prob = 1.0;
    let mut days = 365.0;
    for i in 0..n {
        prob *= (days - i as f64) / days;
    }
    
    (1.0 - prob).round_to(4)
}
