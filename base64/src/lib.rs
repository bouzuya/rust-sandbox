#[derive(Debug)]
pub struct Error(ErrorKind);

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ErrorKind::InvalidCharacter(c) => write!(f, "invalid character: {}", c),
            ErrorKind::InvalidLength(l) => write!(f, "invalid length: {}", l),
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    InvalidCharacter(char),
    InvalidLength(usize),
}

impl From<ErrorKind> for Error {
    fn from(v: ErrorKind) -> Self {
        Error(v)
    }
}

pub fn decode<S: AsRef<str>>(s: S) -> Result<Vec<u8>, Error> {
    let s = s.as_ref();
    if s.len() % 4 != 0 {
        return Err(Error::from(ErrorKind::InvalidLength(s.len())));
    }

    let mut bytes = vec![];
    let mut iter = s.chars();

    // count padding and remove it
    let mut pad_len = 0;
    let mut tail = vec![];
    for _ in 0..2 {
        if let Some(c) = iter.next_back() {
            if c == '=' {
                tail.push('A'); // 'A' = 0b0000_0000 = '='
                pad_len += 1;
            } else {
                tail.push(c);
            }
        }
    }
    let mut iter = iter.chain(tail.into_iter().rev());

    loop {
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(c1), Some(c2), Some(c3), Some(c4)) => {
                let (i1, i2, i3, i4) = (c2i(c1)?, c2i(c2)?, c2i(c3)?, c2i(c4)?);
                bytes.push((i1 & 0b0011_1111) << 2 | (i2 & 0b0011_0000) >> 4);
                bytes.push((i2 & 0b0000_1111) << 4 | (i3 & 0b0011_1100) >> 2);
                bytes.push((i3 & 0b0000_0011) << 6 | (i4 & 0b0011_1111));
            }
            _ => break,
        }
    }

    for _ in 0..pad_len {
        bytes.pop();
    }

    Ok(bytes)
}

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

                s.push(i2c(i1));
                s.push(i2c(i2));
                s.push(i2c(i3));
                s.push(i2c(i4));
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

const CHARS: std::cell::LazyCell<Vec<char>> = std::cell::LazyCell::new(|| {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .chars()
        .collect::<Vec<char>>()
});

fn c2i(c: char) -> Result<u8, Error> {
    Ok(match c {
        'A'..='Z' => c as u8 - b'A',
        'a'..='z' => c as u8 - b'a' + 26,
        '0'..='9' => c as u8 - b'0' + 52,
        '+' => 62,
        '/' => 63,
        _ => return Err(Error::from(ErrorKind::InvalidCharacter(c))),
    })
}

fn i2c(i: u8) -> char {
    CHARS[i as usize]
}
