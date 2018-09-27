use core::iter::Iterator;

pub trait Source: Iterator<Item = u8> {}

impl<T: Iterator<Item = u8>> Source for T {}

#[cfg_attr(feature = "cargo-clippy", allow(option_option))]
pub struct PeekableSource<S: Source> {
    source: S,
    peeked: Option<Option<S::Item>>,
}

impl<S: Source> PeekableSource<S> {
    pub const fn new(s: S) -> Self {
        PeekableSource {
            source: s,
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&S::Item> {
        if self.peeked.is_none() {
            self.consume();
        }

        if let Some(ref x) = self.peeked {
            return x.as_ref();
        }

        unreachable!();
    }

    pub fn consume(&mut self) {
        self.peeked = Some(Iterator::next(&mut self.source));
    }

    pub fn eof(&mut self) -> bool {
        match self.peek() {
            None => true,
            _ => false,
        }
    }
}

impl<S: Source> From<S> for PeekableSource<S> {
    fn from(source: S) -> Self {
        PeekableSource::new(source)
    }
}

pub struct Bytes<'a> {
    s: &'a [u8],
}

impl<'a> Bytes<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        Bytes { s }
    }
}

impl<'a> Iterator for Bytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s.split_first() {
            None => None,
            Some((c, s)) => {
                self.s = s;
                Some(*c)
            }
        }
    }
}

impl<'a> From<&'a [u8]> for PeekableSource<Bytes<'a>> {
    fn from(s: &'a [u8]) -> Self {
        PeekableSource::new(Bytes::new(s))
    }
}

pub trait Sink {
    fn write(&mut self, c: u8);
}