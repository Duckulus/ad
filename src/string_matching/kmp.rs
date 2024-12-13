pub fn kmp_find(text: &str, pattern: &str) -> Option<usize> {
    let prefix_map = compute_prefix_map(pattern);
    let text_chars: Vec<_> = text.chars().collect();
    let pattern_chars: Vec<_> = pattern.chars().collect();
    let mut q = 0;
    for i in 0..text_chars.len() {
        while q > 0 && pattern_chars[q] != text_chars[i] {
            q = prefix_map[q - 1];
        }
        if pattern_chars[q] == text_chars[i] {
            q += 1;
        }
        if q == pattern_chars.len() {
            return Some(i-(pattern.len()-1));
        }
    }
    None
}

fn compute_prefix_map(pattern: &str) -> Vec<usize> {
    let chars: Vec<_> = pattern.chars().collect();
    let mut prefix_map = vec![0; chars.len()];
    prefix_map[0] = 0;
    let mut k = 0;
    for q in 1..chars.len() {
        while k > 0 && chars[k] != chars[q] {
            k = prefix_map[k - 1];
        }
        if chars[k] == chars[q] {
            k += 1;
        }
        prefix_map[q] = k
    }
    prefix_map
}

#[test]
pub fn test_compute_prefix_map() {
    assert_eq!(compute_prefix_map("ababaca"), vec![0, 0, 1, 2, 3, 0, 1]);
    assert_eq!(compute_prefix_map("gegend"), vec![0, 0, 1, 2, 0, 0]);
}

#[test]
pub fn test_kmp_find() {
    assert_eq!(kmp_find("aaab", "aaab"), Some(0));
    assert_eq!(kmp_find("aaaaaaaaaaaaaaaaaaaaab", "ab"), Some(20));
    assert_eq!(kmp_find("the quick brown fox jumps over the lazy dog", "laz"), Some(35));
    assert_eq!(kmp_find("foobar", "baz"), None);
}
