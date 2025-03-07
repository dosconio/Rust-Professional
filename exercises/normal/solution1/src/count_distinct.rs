use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
	// Split the input string by commas and trim whitespace
    let elements: Vec<&str> = input_str.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty()) // Remove empty strings if any
        .collect();
    let mut unique_elements = HashSet::new();
    for element in elements {
        unique_elements.insert(element.to_string());
    }
    unique_elements.len()
}
