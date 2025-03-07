fn num_to_base(mut num: u32, base: u32) -> String {
    let mut result = String::new();
    let digits = "0123456789abcdef";
    
    if num == 0 {
        return "0".to_string();
    }
    
    while num > 0 {
        result.insert(0, digits.chars().nth((num % base) as usize).unwrap());
        num /= base;
    }
    
    result
}


pub fn convert_base(num_str: &str, to_base: u32) -> String {
    if let Some(pos) = num_str.rfind('(') {
        let base_str = &num_str[pos + 1..num_str.len() - 1];
        let num_part = &num_str[..pos];
        if let Ok(from_base) = base_str.parse::<u32>() {
            if let Ok(num) = u32::from_str_radix(num_part, from_base) {
                return num_to_base(num, to_base);
                //return format!("{}({})", num_to_base(num, to_base), to_base);
            }
        }
    }
    
    "Invalid Input".to_string()
}
