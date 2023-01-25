use encoding_rs::{SHIFT_JIS, UTF_8};
use std::io::{BufRead, Cursor, Read, Result};

enum Encording {
    Utf8,
    Sjis,
}

pub struct Binary {
    cursor: Cursor<Vec<u8>>,
}

impl Binary {
    pub fn new(bytes: Vec<u8>) -> Binary {
        Binary {
            cursor: Cursor::new(bytes),
        }
    }

    pub fn get_position(&self) -> u64 {
        self.cursor.position()
    }

    pub fn set_position(&mut self, pos: u64) {
        self.cursor.set_position(pos);
    }

    pub fn increment_position(&mut self, length: u64) {
        let pos = self.cursor.position() + length;
        self.cursor.set_position(pos);
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.cursor.read_exact(&mut buf)?;
        Ok(u8::from_be_bytes(buf))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.cursor.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.cursor.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        self.cursor.read_exact(&mut buf)?;
        Ok(i8::from_be_bytes(buf))
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.cursor.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_utf8(&mut self, length: u8) -> Result<String> {
        self.read_text(length, Encording::Utf8)
    }

    pub fn read_sjis(&mut self, length: u8) -> Result<String> {
        self.read_text(length, Encording::Sjis)
    }

    fn read_text(&mut self, length: u8, encording: Encording) -> Result<String> {
        let mut buf = Vec::new();
        self.cursor.read_until(length, &mut buf)?;

        let text = match encording {
            Encording::Utf8 => {
                let (cow, _, _) = UTF_8.decode(&buf);
                cow.into_owned()
            }
            Encording::Sjis => {
                let (cow, _, _) = SHIFT_JIS.decode(&buf);
                cow.into_owned()
            }
        };

        self.increment_position(length.into());
        Ok(text)
    }

    // pub fn to_bytes(&self) -> <Vec<u8>>{}

    // pub fn read_number<T: Number>(&mut self) -> Result<T> {
    //     // let mut res = [0; T::get_length()];
    //     let mut res = vec![0; T::get_length()];
    //     self.cursor.read_exact(&mut res)?;
    //     Ok(T::from_be_bytes(res))
    // }
}

// trait Number {
//     fn get_length() -> usize;
//     // fn get_length() -> u8 {
//     //     (Self::BITS / 8) as u8
//     // }
//     fn from_be_bytes(bytes: Vec<u8>) -> Self;
//     // fn from_be_bytes(bytes: [u8; 1]) -> Self {
//     //     Self::from_be_bytes(bytes)
//     // }
// }
// impl Number for u8 {
//     fn get_length() -> usize {
//         (Self::BITS / 8) as usize
//         // 1
//     }
//     fn from_be_bytes(bytes: Vec<u8>) -> Self {
//         Self::from_be_bytes(bytes.try_into().unwrap())
//     }
// }
