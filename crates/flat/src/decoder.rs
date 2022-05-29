use crate::decode::Decode;

pub struct Decoder<'b> {
    buffer: &'b [u8],
    pos: usize,
}

impl<'b> Decoder<'b> {
    pub fn new(bytes: &'b [u8]) -> Decoder<'b> {
        Decoder {
            buffer: bytes,
            pos: 0,
        }
    }

    pub fn decode<T: Decode<'b>>(&mut self) -> Result<T, String> {
        T::decode(self)
    }
}