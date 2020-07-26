use std::convert::TryFrom;
use std::str::FromStr;
use std::{fmt, cmp::{Eq, PartialEq}};

#[derive(PartialEq, Eq, fmt::Debug)]
 pub struct ChunkType{
     chunk_type: [u8; 4],
 }

 impl TryFrom<[u8; 4]> for ChunkType{
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error>{
        Ok(ChunkType{chunk_type: value})
    }
 }

 impl FromStr for ChunkType{
     type Err = &'static str;

     fn from_str(s: &str)->Result<Self, Self::Err>{
         if s.len() != 4{
             Err("Chunk type needs 4 letters")
         }else{
            let mut arr: [u8; 4] = [0; 4];
            arr.copy_from_slice(s.as_bytes());
            for byte in &arr{
                if !is_alphabet(byte){
                    return Err("Chunk type should only contain alphabets")
                }
            }
            Ok(ChunkType{chunk_type: arr})
         }

     }
 }

 impl fmt::Display for ChunkType{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
         write!(f, "{}",std::str::from_utf8(&self.chunk_type).unwrap())
     }
 }
 
 impl ChunkType{
     pub fn bytes(&self) -> [u8; 4]{
         self.chunk_type.clone()
     }

     pub fn is_critical(&self) -> bool{
        65 <= self.chunk_type[0] && self.chunk_type[0] <= 90 //first byte should be uppercase
     }

     pub fn is_public(&self) -> bool{
        65 <= self.chunk_type[1] && self.chunk_type[1] <= 90 //second byte should be uppercase
     }


     pub fn is_valid(&self) -> bool{
         let mut all_alphabets = true;
         for val in &self.chunk_type{
            if !is_alphabet(val){
                all_alphabets = false;
                break;
            }
         }
        all_alphabets && self.is_reserved_bit_valid()
     }
     pub fn is_reserved_bit_valid(&self) -> bool{
        65 <= self.chunk_type[2] && self.chunk_type[2] <= 90 //third byte should be uppercase
     }

     pub fn is_safe_to_copy(&self) -> bool{
        97 <= self.chunk_type[3] && self.chunk_type[3] <= 122 //fourth byte should be lowercase
     }
 }

 fn is_alphabet(byte: &u8) -> bool{
    (65 <= *byte && *byte <= 90) ||
    (97 <= *byte && *byte <= 122)
 }


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
}