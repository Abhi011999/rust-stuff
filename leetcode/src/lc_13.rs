use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let s = s.as_str();
    let s_vec: Vec<char> = s.chars().collect();
    let mut hm: HashMap<char, i32> = HashMap::new();

    hm.insert('I', 1);
    hm.insert('V', 5);
    hm.insert('X', 10);
    hm.insert('L', 50);
    hm.insert('C', 100);
    hm.insert('D', 500);
    hm.insert('M', 1000);

    let mut num = 0;
    let mut i = 0;

    while i < s.len() {
        let _s = s_vec[i];
        let mut _ss;

        if i == s.len() - 1 {
            _ss = s_vec[i];
        } else {
            _ss = s_vec[i + 1];
        }

        if hm[&_s] < hm[&_ss] {
            num += (hm[&_s] - hm[&_ss]).abs();
            i += 1;
        } else {
            num += hm[&_s];
        }

        i += 1;
    }

    return num;
}
