/// Parse a string into a vector of i32
/// The string should be like `[-1,2,8,-5,2]`
pub(crate) fn string_to_vec(input: String) -> Vec<i32> {
    input
        .replace("[", "")
        .replace("]", "")
        .split(',')
        .map(|s| s.parse().expect("parse error"))
        .collect()
}