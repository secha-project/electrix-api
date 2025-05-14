use std::ops::Mul;


pub fn to_string<T: ToString>(value: Option<&T>) -> String {
    value
        .as_ref()
        .map_or_else(String::new, |v| v.to_string())
}


pub fn to_string_with_factor<T: Mul<Output = T> + ToString + Copy>(
    value: Option<T>,
    factor: T
) -> String where for<'a> &'a T: Mul<T> {
    value
        .as_ref()
        .map_or_else(String::new, |v| (*v * factor).to_string())
}
