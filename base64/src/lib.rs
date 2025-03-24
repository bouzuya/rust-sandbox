const CHARS: std::cell::LazyCell<Vec<char>> = std::cell::LazyCell::new(|| {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .chars()
        .collect::<Vec<char>>()
});

pub fn encode<I: AsRef<[u8]>>(bytes: I) -> String {
    let bytes = bytes.as_ref();
    let pad_len = (3 - (bytes.len() % 3)) % 3;

    let mut iter = bytes
        .into_iter()
        .chain(std::iter::repeat_n(&0b0000_0000, pad_len));

    let mut s = String::new();
    loop {
        match (iter.next(), iter.next(), iter.next()) {
            (None, None, None) => break,
            (Some(b1), Some(b2), Some(b3)) => {
                let i1 = b1 >> 2;
                let i2 = ((b1 & 0b0000_0011) << 4) | (b2 >> 4);
                let i3 = ((b2 & 0b0000_1111) << 2) | (b3 >> 6);
                let i4 = b3 & 0b0011_1111;

                s.push(CHARS[i1 as usize]);
                s.push(CHARS[i2 as usize]);
                s.push(CHARS[i3 as usize]);
                s.push(CHARS[i4 as usize]);
            }
            _ => unreachable!(),
        }
    }
    for _ in 0..pad_len {
        s.pop();
    }
    for _ in 0..pad_len {
        s.push('=');
    }

    s
}
