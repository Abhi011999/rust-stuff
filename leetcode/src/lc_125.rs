pub fn is_palindrome(s: String) -> bool {
    let s = &s.replace(" ", "").to_lowercase();
    let s: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let r_s: String = s.chars().rev().collect();
    s.eq(&r_s)
}
