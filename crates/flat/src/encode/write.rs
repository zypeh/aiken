use super::error;

#[derive(Debug, Clone)]
pub struct Writer<W> {
    writer: W,
}

// impl<W: Write> Encoder<W> {
//     /// Construct an `Encoder` that writes to the given [`Write`] sink.
//     pub fn new(writer: W) -> Encoder<W> {
//         Encoder { writer }
//     }

//     pub fn writer(Encoder { writer }: Self) -> W {
//         writer
//     }
// }

pub trait Write {
    type Error;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

impl Write for &mut [u8] {
    type Error = error::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        // this code will write all data from the writer buffer to buf arg passed
        if self.len() < buf.len() {
            return Err(error::Error::EndOfSlice);
        }
        let this = core::mem::take(self);
        let (prefix, suffix) = this.split_at_mut(buf.len());
        prefix.copy_from_slice(buf);
        *self = suffix;
        Ok(())
    }
}

// this is the default way of flat-rs.
impl Write for &mut Vec<u8> {
    type Error = error::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

// #[cfg(test)]
// mod test {
//     use super::Encoder;

//     #[test]
//     fn test() {
//         let mut buffer = [0u8; 128];
//         println!("{:?}", Encoder::new(&mut buffer[..]));
//     }

//     #[test]
//     fn test2() {
//         let mut buffer = vec![];
//         println!("{:?}", Encoder::new(&mut buffer));
//     }
// }
