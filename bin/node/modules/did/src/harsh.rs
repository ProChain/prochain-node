use rstd::vec::Vec;
use rstd::prelude::{Box};

const DEFAULT_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const DEFAULT_SEPARATORS: &[u8] = b"cfhistuCFHISTU";
const SEPARATOR_DIV: f64 = 3.5;
const GUARD_DIV: f64 = 12.0;
const MINIMUM_ALPHABET_LENGTH: usize = 16;

/// A hashids-compatible hasher.
///
/// It's probably not a great idea to use the default, because in that case
/// your values will be entirely trivial to _decode. On the other hand, this is
/// not intended to be cryptographically-secure, so go nuts!
#[derive(Clone, Debug)]
pub struct Harsh {
    salt: Box<[u8]>,
    _alphabet: Box<[u8]>,
    separators: Box<[u8]>,
    hash_length: usize,
    guards: Box<[u8]>,
}

impl Harsh {
    /// Encodes a slice of `u64` values into a single hashid.
    pub fn encode(&self, values: &[u64]) -> Option<Vec<u8>> {
        if values.is_empty() {
            return None;
        }

        let nhash = create_nhash(values);

        let mut _alphabet = self._alphabet.clone();
        let mut buffer = Vec::new();

        let idx = (nhash % _alphabet.len() as u64) as usize;
        let lottery = _alphabet[idx];
        buffer.push(lottery);

        for (idx, &value) in values.iter().enumerate() {
            let mut value = value;

            let temp = {
                let mut temp = Vec::with_capacity(self.salt.len() + _alphabet.len() + 1);
                temp.push(lottery);
                temp.extend_from_slice(&self.salt);
                temp.extend_from_slice(&_alphabet);
                temp
            };

            let alphabet_len = _alphabet.len();
            shuffle(&mut _alphabet, &temp[..alphabet_len]);

            let last = hash(value, &_alphabet);
            buffer.append(&mut last.clone());

            if idx + 1 < values.len() {
                value %= (last[0] as usize + idx) as u64;
                buffer
                    .push(self.separators[(value % self.separators.len() as u64) as usize]);
            }
        }

        if buffer.len() < self.hash_length {
            let guard_index = (nhash as usize
                + buffer[0] as usize)
                % self.guards.len();
            let guard = self.guards[guard_index];
            buffer.insert(0, guard);

            if buffer.len() < self.hash_length {
                let guard_index = (nhash as usize
                    + buffer[2] as usize)
                    % self.guards.len();
                let guard = self.guards[guard_index];
                buffer.push(guard);
            }
        }

        let half_length = _alphabet.len() / 2;
        while buffer.len() < self.hash_length {
            {
                let alphabet_copy = _alphabet.clone();
                shuffle(&mut _alphabet, &alphabet_copy);
            }

            let (left, right) = _alphabet.split_at(half_length);
            
            buffer = [left, &buffer[..], right].concat();

            let excess = buffer.len() as i32 - self.hash_length as i32;
            if excess > 0 {
                let marker = excess as usize / 2;
                buffer = buffer[marker..marker + self.hash_length].to_vec();
            }
        }

        Some(buffer)
    }

    /// Decodes a single hashid into a slice of `u64` values.
    pub fn _decode(&self, value: &[u8]) -> Option<Vec<u64>> {
        let mut value = value.as_ref().to_vec();

        if let Some(guard_idx) = value.iter().rposition(|u| self.guards.contains(u)) {
            value.truncate(guard_idx);
        }

        let value = match value.iter().position(|u| self.guards.contains(u)) {
            None => &value[..],
            Some(guard_idx) => &value[(guard_idx + 1)..],
        };

        if value.len() < 2 {
            return None;
        }

        let mut _alphabet = self._alphabet.clone();

        let lottery = value[0];
        let value = &value[1..];
        let segments: Vec<_> = value.split(|u| self.separators.contains(u)).collect();

        segments
            .into_iter()
            .map(|segment| {
                let buffer = {
                    let mut buffer = Vec::with_capacity(self.salt.len() + _alphabet.len() + 1);
                    buffer.push(lottery);
                    buffer.extend_from_slice(&self.salt);
                    buffer.extend_from_slice(&_alphabet);
                    buffer
                };

                let alphabet_len = _alphabet.len();
                shuffle(&mut _alphabet, &buffer[..alphabet_len]);
                _unhash(segment, &_alphabet)
            })
            .collect()
    }
}

impl Default for Harsh {
    fn default() -> Harsh {
        HarshBuilder::new().init().unwrap()
    }
}

/// A builder used to configure and create a Harsh instance.
#[derive(Debug, Default)]
pub struct HarshBuilder {
    salt: Option<Vec<u8>>,
    _alphabet: Option<Vec<u8>>,
    separators: Option<Vec<u8>>,
    hash_length: usize,
}

impl HarshBuilder {
    /// Creates a new `HarshBuilder` instance.
    pub fn new() -> HarshBuilder {
        HarshBuilder {
            salt: None,
            _alphabet: None,
            separators: None,
            hash_length: 0,
        }
    }

    /// Provides a salt.
    ///
    /// Note that this salt will be converted into a `[u8]` before use, meaning
    /// that multi-byte utf8 character values should be avoided.
    pub fn salt<T: Into<Vec<u8>>>(mut self, salt: T) -> HarshBuilder {
        self.salt = Some(salt.into());
        self
    }

    /// Provides an _alphabet.
    ///
    /// Note that this _alphabet will be converted into a `[u8]` before use, meaning
    /// that multi-byte utf8 character values should be avoided.
    pub fn _alphabet<T: Into<Vec<u8>>>(mut self, _alphabet: T) -> HarshBuilder {
        self._alphabet = Some(_alphabet.into());
        self
    }

    /// Provides a set of separators.
    ///
    /// Note that these separators will be converted into a `[u8]` before use,
    /// meaning that multi-byte utf8 character values should be avoided.
    pub fn _separators<T: Into<Vec<u8>>>(mut self, separators: T) -> HarshBuilder {
        self.separators = Some(separators.into());
        self
    }

    /// Provides a minimum hash length.
    ///
    /// Keep in mind that hashes produced may be longer than this length.
    pub fn length(mut self, hash_length: usize) -> HarshBuilder {
        self.hash_length = hash_length;
        self
    }

    /// Initializes a new `Harsh` based on the `HarshBuilder`.
    ///
    /// This method will consume the `HarshBuilder`.
    pub fn init(self) -> Result<Harsh, &'static str> {
        let _alphabet = unique_alphabet(&self._alphabet)?;
        if _alphabet.len() < MINIMUM_ALPHABET_LENGTH {
            return Err("_alphabet length error");
        }

        let salt = self.salt.unwrap_or_else(Vec::new);
        let (mut _alphabet, mut separators) =
            alphabet_and_separators(&self.separators, &_alphabet, &salt);
        let guards = guards(&mut _alphabet, &mut separators);

        Ok(Harsh {
            salt: salt.into_boxed_slice(),
            _alphabet: _alphabet.into_boxed_slice(),
            separators: separators.into_boxed_slice(),
            hash_length: self.hash_length,
            guards: guards.into_boxed_slice(),
        })
    }
}

#[inline]
fn create_nhash(values: &[u64]) -> u64 {
    values
        .iter()
        .enumerate()
        .fold(0, |a, (idx, value)| a + (value % (idx + 100) as u64))
}

fn unique_alphabet(_alphabet: &Option<Vec<u8>>) -> Result<Vec<u8>, &'static str> {

    match *_alphabet {
        None => {
            let mut vec = [0; 62];
            vec.clone_from_slice(DEFAULT_ALPHABET);
            Ok(vec.to_vec())
        }

        Some(ref _alphabet) => {
            let mut reg = Vec::new();
            let mut ret = Vec::new();

            for (idx, &item) in _alphabet.iter().enumerate() {
                if item == b' ' {
                    return Err("illegal words");
                }

                if !reg.contains(&item) {
                    ret.push(item);
                    reg.insert(idx, item);
                }
            }

            if ret.len() < 16 {
                Err("length error")
            } else {
                Ok(ret)
            }
        }
    }
}

fn alphabet_and_separators(
    separators: &Option<Vec<u8>>,
    _alphabet: &[u8],
    salt: &[u8],
) -> (Vec<u8>, Vec<u8>) {
    let separators = match *separators {
        None => DEFAULT_SEPARATORS,
        Some(ref separators) => separators,
    };

    let mut separators: Vec<_> = separators
        .iter()
        .cloned()
        .filter(|item| _alphabet.contains(item))
        .collect();
    let mut _alphabet: Vec<_> = _alphabet
        .iter()
        .cloned()
        .filter(|item| !separators.contains(item))
        .collect();

    shuffle(&mut separators, salt);

    if separators.is_empty() || (_alphabet.len() as f64 / separators.len() as f64) > SEPARATOR_DIV {
        let length = match (_alphabet.len() as f64 / SEPARATOR_DIV) as usize {
            1 => 2,
            n => n,
        };

        if length > separators.len() {
            let diff = length - separators.len();
            separators.extend_from_slice(&_alphabet[..diff]);
            _alphabet = _alphabet[diff..].to_vec();
        } else {
            separators = separators[..length].to_vec();
        }
    }

    shuffle(&mut _alphabet, salt);
    (_alphabet, separators)
}

fn guards(_alphabet: &mut Vec<u8>, separators: &mut Vec<u8>) -> Vec<u8> {
    let guard_count = (_alphabet.len() as f64 / GUARD_DIV) as usize;
    // let guard_count = (_alphabet.len() as f64 / GUARD_DIV).ceil() as usize;
    if _alphabet.len() < 3 {
        let guards = separators[..guard_count].to_vec();
        separators.drain(..guard_count);
        guards
    } else {
        let guards = _alphabet[..guard_count].to_vec();
        _alphabet.drain(..guard_count);
        guards
    }
}

fn shuffle(values: &mut [u8], salt: &[u8]) {
    if salt.is_empty() {
        return;
    }

    let values_length = values.len();
    let salt_length = salt.len();
    let (mut v, mut p) = (0, 0);

    for i in (1..values_length).map(|i| values_length - i) {
        v %= salt_length;

        let n = salt[v] as usize;
        p += n;
        let j = (n + v + p) % i;

        values.swap(i, j);
        v += 1;
    }
}

fn hash(mut value: u64, _alphabet: &[u8]) -> Vec<u8> {
    let length = _alphabet.len() as u64;
    let mut hash = Vec::new();

    loop {
        hash.push(_alphabet[(value % length) as usize]);
        value /= length;

        if value == 0 {
            hash.reverse();
            return hash;
        }
    }
}

fn _unhash(input: &[u8], _alphabet: &[u8]) -> Option<u64> {
    input.iter().enumerate().fold(Some(0), |a, (idx, &value)| {
        let pos = _alphabet.iter().position(|&item| item == value)? as u64;
        a.map(|a| a + (pos * (_alphabet.len() as u64).pow((input.len() - idx - 1) as u32)))
    })
}

#[cfg(test)]
mod tests {
    use super::{Harsh, HarshBuilder};

    #[test]
    fn harsh_default_does_not_panic() {
        Harsh::default();
    }

    #[test]
    fn can_encode() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .init()
            .expect("failed to initialize harsh");

        let result = harsh.encode(&[1226198605112]).expect("failed to encode");
        let strs = rstd::str::from_utf8(&result).unwrap();
        println!("result is {}", strs);
        assert_eq!(
            b"4o6Z7KqxE",
            result.as_slice(),
            "error encoding [1226198605112]"
        );

        assert_eq!(
            b"laHquq",
            harsh.encode(&[1, 2, 3]).expect("failed to encode").as_slice()
        );
    }

    #[test]
    fn can_encode_with_guards() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(8)
            .init()
            .expect("failed to initialize harsh");

        assert_eq!(
            b"GlaHquq0",
            harsh.encode(&[1, 2, 3]).expect("failed to encode").as_slice()
        );
    }

    #[test]
    fn can_encode_with_padding() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(12)
            .init()
            .expect("failed to initialize harsh");

        let encode = harsh.encode(&[1, 2, 3]).expect("failed to encode");
        println!("encode---- is {}", rstd::str::from_utf8(&encode).unwrap());

        // 9LGlaHquq06D 
        assert_eq!(
            b"1vGlaHquq0Ba",
            harsh.encode(&[1, 2, 3]).expect("failed to encode").as_slice()
        );
    }

    #[test]
    fn can_decode() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .init()
            .expect("failed to initialize harsh");

        assert_eq!(
            &[1226198605112],
            &harsh._decode(b"4o6Z7KqxE").expect("failed to _decode")[..],
            "error decoding \"4o6Z7KqxE\""
        );
        assert_eq!(
            &[1u64, 2, 3],
            &harsh._decode(b"laHquq").expect("failed to _decode")[..]
        );
    }

    #[test]
    fn can_decode_with_guards() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(8)
            .init()
            .expect("failed to initialize harsh");

        assert_eq!(
            &[1u64, 2, 3],
            &harsh._decode(b"GlaHquq0").expect("failed to _decode")[..]
        );
    }

    #[test]
    fn can_decode_with_padding() {
        let harsh = HarshBuilder::new()
            .salt("this is my salt")
            .length(12)
            .init()
            .expect("failed to initialize harsh");

        assert_eq!(
            &[1u64, 2, 3],
            &harsh._decode(b"9LGlaHquq06D").expect("failed to _decode")[..]
        );
    }

    #[test]
    fn can_encode_with_custom_alphabet() {
        let harsh = HarshBuilder::new()
            ._alphabet("abcdefghijklmnopqrstuvwxyz")
            .init()
            .expect("failed to initialize harsh");
        
        // mdfphx
        assert_eq!(
            b"lqfqhr",
            harsh.encode(&[1, 2, 3]).expect("failed to encode").as_slice(),
            "failed to encode [1, 2, 3]"
        );
    }

    #[test]
    fn can_decode_with_invalid_alphabet() {
        let harsh = Harsh::default();
        assert_eq!(None, harsh._decode(b"this$ain't|a\number"));
    }

    #[test]
    fn can_decode_with_custom_alphabet() {
        let harsh = HarshBuilder::new()
            ._alphabet("abcdefghijklmnopqrstuvwxyz")
            .init()
            .expect("failed to initialize harsh");

        // mdfphx
        assert_eq!(
            &[1, 2, 3],
            &harsh._decode(b"lqfqhr").expect("failed to _decode")[..],
            "failed to _decode lqfqhr"
        );
    }

    #[test]
    fn create_nhash() {
        let values = &[1, 2, 3];
        let nhash = super::create_nhash(values);
        assert_eq!(6, nhash);
    }

    #[test]
    fn hash() {
        let result = super::hash(22, b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(b"w", result.as_slice());
    }

    #[test]
    fn alphabet_and_separator_generation() {
        use super::{DEFAULT_ALPHABET, DEFAULT_SEPARATORS};

        let (_alphabet, separators) = super::alphabet_and_separators(
            &Some(DEFAULT_SEPARATORS.to_vec()),
            DEFAULT_ALPHABET,
            b"this is my salt",
        );

        assert_eq!(
            b"AdG05N6y2rljDQak4xgzn8ZR1oKYLmJpEbVq3OBv9WwXPMe7".to_vec(),
            _alphabet.as_slice()
        );
        assert_eq!(
            b"UHuhtcITCsFifS",
            separators.as_slice()
        );
    }

    #[test]
    fn alphabet_and_separator_generation_with_few_separators() {
        use super::DEFAULT_ALPHABET;

        let separators = b"fu";
        let (_alphabet, separators) = super::alphabet_and_separators(
            &Some(separators.to_vec()),
            DEFAULT_ALPHABET,
            b"this is my salt",
        );

        // 4RVQrYM87wKPNSyTBGU1E6FIC9ALtH0ZD2Wxz3vs5OXJ
        assert_eq!(
            b"57148IVEH6sKq2B3UAtXYLyRMwvJCWNrSxzZFOG9TQ0PD".to_vec(),
            _alphabet
        );

        // ufabcdeghijklmnopq
        assert_eq!(
            b"ufabcdeghijklmnop".to_vec(),
            separators
        );
    }

    #[test]
    fn shuffle() {
        let salt = b"1234";
        let mut values = "asdfzxcvqwer".bytes().collect::<Vec<_>>();
        super::shuffle(&mut values, salt);

        assert_eq!(b"vdwqfrzcsxae", &values.as_slice());
    }
}