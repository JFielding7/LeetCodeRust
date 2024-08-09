const ANY_SEQ: u8 = '*' as u8;
const ANY_CHAR: u8 = '?' as u8;

fn match_end(string: &[u8], mut i: usize, string_length: usize, pattern: &[u8], pattern_start: usize, pattern_end: usize) -> Option<usize> {
    if pattern_start == pattern_end { return Option::from(i); }
    while i < string_length {
        let mut a = i;
        let mut b = pattern_start;
        while string[a] == pattern[b] || pattern[b] == ANY_CHAR {
            a += 1;
            b += 1;
            if b == pattern_end { return Option::from(a); }
            if a == string_length { return None; }
        }
        i += 1;
    }
    None
}

fn exact_match(string: &[u8], mut string_curr: usize, string_end: usize, pattern: &[u8], mut pattern_curr: usize) -> bool {
    while string_curr < string_end {
        if string[string_curr] != pattern[pattern_curr] && pattern[pattern_curr] != ANY_CHAR { return false; }
        string_curr += 1;
        pattern_curr += 1;
    }
    return true;
}

pub fn is_match(s: String, p: String) -> bool {
    let string = s.as_bytes();
    let string_length = string.len();
    let pattern = p.as_bytes();
    let pattern_length = pattern.len();
    let mut i = 0;
    let mut j = 0;
    while j < pattern_length && pattern[j] != ANY_SEQ { j += 1; }
    if j > string_length || (j == pattern_length && string_length != pattern_length) || !exact_match(string, i, j, pattern, 0) { return false; }
    let mut pattern_start = 0;
    while j < pattern_length {
        if pattern[j] == ANY_SEQ {
            let end = match_end(string, i, string_length, pattern, pattern_start, j);
            if end.is_none() { return false; }
            i = end.unwrap();
            pattern_start = j + 1;
        }
        j += 1;
    }
    if string_length + pattern_start < pattern_length { return false; }
    let string_curr = string_length + pattern_start - pattern_length;
    return string_curr >= i && exact_match(string, string_curr, string_length, pattern, pattern_start);
}