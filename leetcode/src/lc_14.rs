pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut word_vec = strs;
    let mut prefix = String::new();
    let mut temp_prefix = String::new();

    word_vec.sort_by(|a, b| a.len().cmp(&b.len()));

    for fw_ch in word_vec[0].chars() {
        temp_prefix.push(fw_ch);
        for j in 1..word_vec.len() {
            if word_vec[j].starts_with(&temp_prefix) {
                continue;
            } else {
                return prefix;
            }
        }
        prefix = temp_prefix.clone();
    }
    
    return prefix;
}