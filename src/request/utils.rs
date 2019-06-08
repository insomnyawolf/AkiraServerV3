use crate::utils::log::log_warning;

/// Check if the string estarts with the defined pattern
/// If starts with the pattern, removes the pattern and stores the remeaning data to the field
/// as `` String ``
pub fn generate_field_string(data: &str) -> String {
    return data.to_string();
}

/// Check if the string estarts with the defined pattern
/// If starts with the pattern, removes the pattern and stores the remeaning data to the field
/// as `` Vec<String> ``
pub fn generate_field_string_vec(data: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let da: Vec<&str> = data.split(" ").collect();
    for d in da {
        v.push(d.to_string());
    }
    return v;
}

/// Check if the string estarts with the defined pattern
/// If starts with the pattern, removes the pattern and stores the remeaning data to the field
/// as `` u64 ``
pub fn generate_field_u64(data: &str) -> u64 {
    return match data.parse::<u64>() {
        Ok(value) => value,
        Err(err) => {
            log_warning(&err);
            0
        },
    };
}

/// Check if the string estarts with the defined pattern
/// If starts with the pattern, removes the pattern and stores the remeaning data to the field
/// as `` [u8] ``
pub fn generate_field_vec_u8(data: &str) -> Vec<u8> {
    data.as_bytes().to_owned()
}

pub fn start_with(data: &str, pattern: &str) -> bool {
    if data.to_lowercase().starts_with(&pattern.to_lowercase()[..]) {
        return true;
    }
    false
}
