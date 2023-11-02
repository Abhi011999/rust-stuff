use std::collections::HashMap;

// not efficient but works
pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut hm: HashMap<char, char> = HashMap::new();
    let mut rhm: HashMap<char, char> = HashMap::new();

    for i in 0..s.len() {
        let s_char = &s.chars().nth(i).unwrap();
        let t_char = &t.chars().nth(i).unwrap();

        if hm.contains_key(s_char) {
            if hm[s_char].ne(t_char) {
                return false;
            }
        }

        if rhm.contains_key(t_char) {
            if rhm[t_char].ne(s_char) {
                return false;
            }
        }

        hm.insert(*s_char, *t_char);
        rhm.insert(*t_char, *s_char);
    }

    true
}
