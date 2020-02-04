mod block;

/// ```
/// # use any_ascii::any_ascii;
/// assert_eq!("anthropoi", any_ascii("άνθρωποι"));
/// ```
pub fn any_ascii(s: &str) -> String {
    s.chars().map(any_ascii_char).collect()
}

/// ```
/// # use any_ascii::any_ascii_char;
/// assert_eq!("ae", any_ascii_char('æ'));
/// ```
pub fn any_ascii_char(c: char) -> &'static str {
    let block_num = (c as u32) >> 8;
    let b = block::block(block_num);
    let block: &'static [[u8; 3]] = unsafe {
        std::slice::from_raw_parts(b.as_ptr() as *const [u8; 3], b.len() / 3)
    };
    let lo = (c as u8) as usize;
    if let Some(p) = block.get(lo) {
        let mut len = p[2] as usize;
        if len >= 32 {
            len = 3
        }
        if len <= 3 {
            unsafe {
                std::str::from_utf8_unchecked(&p[..len])
            }
        } else {
            let i = (((p[0] as u16) << 8) | (p[1] as u16)) as usize;
            unsafe {
                include_str!("strings.txt").get_unchecked(i..i + len)
            }
        }
    } else {
        ""
    }
}

#[test]
fn test() {
    fn check(s: &str, expected: &str) {
        assert_eq!(any_ascii(s), expected)
    }
    check("", "");
    check("a", "a");
    check("123", "123");
    check("άνθρωποι", "anthropoi");
    check("北亰", "BeiJing");
    check("résumé", "resume");
}