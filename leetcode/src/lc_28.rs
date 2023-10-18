pub fn str_str(haystack: String, needle: String) -> i32 {
    let ch2_n1 = needle.chars().nth(0).unwrap();
    'first: for (i, ch1) in haystack.chars().enumerate() {
        if ch1 == ch2_n1 {
            for (j, ch2) in needle.chars().enumerate() {
                if i + j < haystack.len() && haystack.chars().nth(i + j).unwrap() == ch2 {
                    continue;
                } else {
                    continue 'first;
                }
            }
            return i as i32;
        }
    }
    -1
}
