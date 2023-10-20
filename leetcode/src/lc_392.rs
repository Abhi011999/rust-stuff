pub fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;

    let sl = s.len();
    let tl = t.len();

    while i < sl && j < tl {
        if s.chars().nth(i) == t.chars().nth(j) {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    if i == sl {
        return true;
    }

    false
}
