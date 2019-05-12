/**
Check if the string estarts with the defined pattern
If starts with the pattern, removes the pattern and stores the remeaning data to the field
as
```
String
```
**/
pub fn generate_field_string(field: &mut String, data: &str, pattern: &str) -> bool {
    if data.to_lowercase().starts_with(&pattern.to_lowercase()[..]) {
        *field = data[pattern.len()..].to_string();
        return true;
    }
    false
}

/**
Check if the string estarts with the defined pattern
If starts with the pattern, removes the pattern and stores the remeaning data to the field
as
```
Vec<String>
```
**/
pub fn generate_field_string_vec(field: &mut Vec<String>, data: &str, pattern: &str) -> bool {
    if data.to_lowercase().starts_with(&pattern.to_lowercase()[..]) {
        let s = data[pattern.len()..].to_string();
        let values: Vec<&str> = s.split(" ").collect();
        for value in values {
            field.push(value.to_string());
        }
        return true;
    }
    false
}

/**
Check if the string estarts with the defined pattern
If starts with the pattern, removes the pattern and stores the remeaning data to the field
as
```
u64
```
**/
pub fn generate_field_u64(field: &mut u64, data: &str, pattern: &str) -> bool {
    if data.to_lowercase().starts_with(&pattern.to_lowercase()[..]) {
        *field = data[pattern.len()..].parse::<u64>().unwrap();
        return true;
    }
    false
}

pub fn generate_field_vec_u8(field: &mut Vec<u8>, data: &str) {
    *field = data.as_bytes().to_owned();
}
