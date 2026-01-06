pub fn valid_int(value: &str) -> bool {
    value.parse::<i32>().is_ok()
}

pub fn valid_str(value: &str) -> bool {
    if value.ends_with("'") & value.starts_with("'") {
        let trimmed_value = value.trim_start_matches("'").trim_end_matches("'");
        if trimmed_value.parse::<String>().is_ok() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn valid_float(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

pub fn is_constant(expr: &str) -> bool {
    let state = expr.split_whitespace().nth(0).unwrap();
    if state == "const" {
        return true;
    } else {
        return false;
    }
}

