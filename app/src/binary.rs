use encoding_rs::{SHIFT_JIS, UTF_16BE, UTF_8};
use std::io::{Cursor, Read, Result};

enum Encording {
    Utf8,
    Utf16,
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

    pub fn read_f64(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        self.cursor.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }

    pub fn read_utf8(&mut self, length: u8) -> Result<String> {
        self.read_text(length as usize, Encording::Utf8)
    }

    // pub fn read_utf8_pascal_string(&mut self) -> Result<String> {
    //     let length = self.read_u8()?;
    //     self.read_text(length as usize, Encording::Utf8)
    // }

    // pub fn read_sjis(&mut self, length: u8) -> Result<String> {
    //     self.read_text(length as usize, Encording::Sjis)
    // }

    pub fn read_sjis_pascal_string(&mut self) -> Result<String> {
        let length = self.read_u8()?;
        self.read_text(length as usize, Encording::Sjis)
    }

    pub fn read_unicode_string(&mut self) -> Result<String> {
        let count = self.read_u32()?;
        let length = count * 2;
        self.read_text(length as usize, Encording::Utf16)
    }

    fn read_text(&mut self, length: usize, encording: Encording) -> Result<String> {
        let mut buf = vec![0; length];
        self.cursor.read_exact(&mut buf)?;

        let (cow, _, _) = match encording {
            Encording::Utf8 => UTF_8.decode(&buf),
            Encording::Utf16 => UTF_16BE.decode(&buf),
            Encording::Sjis => SHIFT_JIS.decode(&buf),
        };

        Ok(cow.into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::Binary;

    #[test]
    fn read_u8() {
        let bytes = vec![0, 1, 255];

        let mut binary = Binary::new(bytes);
        let mut n = binary.read_u8().unwrap();
        assert_eq!(n, 0);
        n = binary.read_u8().unwrap();
        assert_eq!(n, 1);
        n = binary.read_u8().unwrap();
        assert_eq!(n, 255);
    }

    #[test]
    fn read_u16() {
        let mut bytes = Vec::new();
        bytes.append(&mut vec![0, 0]);
        bytes.append(&mut vec![0, 1]);
        bytes.append(&mut vec![0, 255]);
        bytes.append(&mut vec![1, 0]);
        bytes.append(&mut vec![255, 0]);
        bytes.append(&mut vec![255; 2]);

        let mut binary = Binary::new(bytes);
        let mut n = binary.read_u16().unwrap();
        assert_eq!(n, 0);
        n = binary.read_u16().unwrap();
        assert_eq!(n, 1);
        n = binary.read_u16().unwrap();
        assert_eq!(n, 255);
        n = binary.read_u16().unwrap();
        assert_eq!(n, 256);
        n = binary.read_u16().unwrap();
        assert_eq!(n, 65535 - 255);
        n = binary.read_u16().unwrap();
        assert_eq!(n, 65535);
    }

    #[test]
    fn read_u32() {
        let mut bytes = Vec::new();
        bytes.append(&mut vec![0, 0, 0, 1]);
        bytes.append(&mut vec![255; 4]);

        let mut binary = Binary::new(bytes);
        let mut n = binary.read_u32().unwrap();
        assert_eq!(n, 1);
        n = binary.read_u32().unwrap();
        assert_eq!(n, 4294967295);
    }

    #[test]
    fn read_i8() {
        let mut bytes = Vec::new();
        bytes.append(&mut vec![0]);
        bytes.append(&mut vec![127]);
        bytes.append(&mut vec![128]);
        bytes.append(&mut vec![255]);

        let mut binary = Binary::new(bytes);
        let mut n = binary.read_i8().unwrap();
        assert_eq!(n, 0);
        n = binary.read_i8().unwrap();
        assert_eq!(n, 127);
        n = binary.read_i8().unwrap();
        assert_eq!(n, -128);
        n = binary.read_i8().unwrap();
        assert_eq!(n, -1);
    }

    #[test]
    fn read_i16() {
        let mut bytes = Vec::new();
        bytes.append(&mut vec![0x00; 2]);
        bytes.append(&mut vec![0x00, 0xFF]);
        bytes.append(&mut vec![0xFF, 0x00]);
        bytes.append(&mut vec![0xFF; 2]);

        let mut binary = Binary::new(bytes);
        let mut n = binary.read_i16().unwrap();
        assert_eq!(n, 0);
        n = binary.read_i16().unwrap();
        assert_eq!(n, 255);
        n = binary.read_i16().unwrap();
        assert_eq!(n, -256);
        n = binary.read_i16().unwrap();
        assert_eq!(n, -1);
    }

    #[test]
    fn read_f64() {
        let mut bytes = Vec::new();
        bytes.append(&mut vec![0x00; 8]);
        bytes.append(&mut vec![0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        bytes.append(&mut vec![0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        bytes.append(&mut vec![0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
        bytes.append(&mut vec![0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
        bytes.append(&mut vec![0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        bytes.append(&mut vec![0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        bytes.append(&mut vec![0x7F, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        bytes.append(&mut vec![0xFF, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        bytes.append(&mut vec![0x7F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        bytes.append(&mut vec![0xFF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let mut binary = Binary::new(bytes);
        let mut n = binary.read_f64().unwrap();
        assert_eq!(n, 0.0);
        n = binary.read_f64().unwrap();
        assert_eq!(n, -0.0);
        n = binary.read_f64().unwrap();
        assert_eq!(n, 1.0);
        n = binary.read_f64().unwrap();
        assert_eq!(n, 1.0000000000000002);
        n = binary.read_f64().unwrap();
        assert_eq!(n, 1.0000000000000004);
        n = binary.read_f64().unwrap();
        assert_eq!(n, 2.0);
        n = binary.read_f64().unwrap();
        assert_eq!(n, -2.0);
        n = binary.read_f64().unwrap();
        assert_eq!(n, f64::MAX);
        n = binary.read_f64().unwrap();
        assert_eq!(n, f64::MIN);
        n = binary.read_f64().unwrap();
        assert_eq!(n, f64::INFINITY);
        n = binary.read_f64().unwrap();
        assert_eq!(n, f64::NEG_INFINITY);
    }

    #[test]
    fn read_utf8() {
        let mut bytes = Vec::new();
        bytes.append(&mut vec![
            0xE3, 0x83, 0x86, 0xE3, 0x82, 0xB9, 0xE3, 0x83, 0x88, 0xE7, 0x94, 0xA8, 0xE6, 0x96,
            0x87, 0xE5, 0xAD, 0x97, 0xE5, 0x88, 0x97,
        ]);

        let mut binary = Binary::new(bytes);
        let text = binary.read_utf8(21).unwrap();
        assert_eq!(text, "テスト用文字列");
    }

    // #[test]
    // fn read_utf8_pascal_string() {
    //     let mut bytes = Vec::new();
    //     bytes.append(&mut vec![
    //         0x15, 0xE3, 0x83, 0x86, 0xE3, 0x82, 0xB9, 0xE3, 0x83, 0x88, 0xE7, 0x94, 0xA8, 0xE6,
    //         0x96, 0x87, 0xE5, 0xAD, 0x97, 0xE5, 0x88, 0x97,
    //     ]);

    //     let mut binary = Binary::new(bytes);
    //     let text = binary.read_utf8_pascal_string().unwrap();
    //     assert_eq!(text, "テスト用文字列");
    // }

    #[test]
    fn read_sjis_pascal_string() {
        let mut bytes = Vec::new();
        bytes.append(&mut vec![
            0x0E, 0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x97, 0x70, 0x95, 0xB6, 0x8E, 0x9A, 0x97,
            0xF1,
        ]);

        let mut binary = Binary::new(bytes);
        let text = binary.read_sjis_pascal_string().unwrap();
        assert_eq!(text, "テスト用文字列");
    }

    #[test]
    fn read_unicode_string() {
        let mut bytes = Vec::new();
        bytes.append(&mut vec![
            0x00, 0x00, 0x00, 0x07, 0x30, 0xC6, 0x30, 0xB9, 0x30, 0xC8, 0x75, 0x28, 0x65, 0x87,
            0x5B, 0x57, 0x52, 0x17,
        ]);

        let mut binary = Binary::new(bytes);
        let text = binary.read_unicode_string().unwrap();
        assert_eq!(text, "テスト用文字列");
    }
}
