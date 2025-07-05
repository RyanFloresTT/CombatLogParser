pub fn extract_timestamp(line: &str) -> String {
    line.split_whitespace().next().unwrap_or("???").to_string()
}
