pub trait ByteReader {
    fn next(&mut self) -> Option<u8>;
}

pub struct ByteSlice<'a> {
    a: &'a [u8],
}

impl<'a> ByteSlice<'a> {
    pub fn from(a: &'a [u8]) -> Self {
        Self {
            a,
        }
    }
}

impl<'a> ByteReader for ByteSlice<'a> {
    fn next(&mut self) -> Option<u8> {
        if self.a.len() == 0 {
            return None;
        }
        let b = self.a[0];
        self.a = &self.a[1..]; 
        Some(b)
    }
}
