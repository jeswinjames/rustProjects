use crate::chunk_type::ChunkType;
use crc::crc32::checksum_ieee;
use std::str::from_utf8;
use std::convert::{TryInto, TryFrom};
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    crc: u32,
    chunk_data: Vec<u8>// when unknown sizes are involved in struct use a Box or reference
}

impl TryFrom<&[u8]> for Chunk{
    type Error = &'static str; // &'static str use this for error strings 
    fn try_from(val : &[u8])-> Result<Self, Self::Error>{
        let error = Err("Conversion error");
        let mut length_bytes : [u8; 4] = [0; 4];
        length_bytes.clone_from_slice(&val[0..4]);
        let chunk_type_bytes : [u8; 4] = match val[4..8].try_into(){
            Ok(val) => val,
            Err(_) => return error
        };
        let chunk_type : ChunkType = match ChunkType::try_from(chunk_type_bytes){
            Ok(val) => val,
            Err(_) => return error
        };
        let length = u32::from_be_bytes(length_bytes);
        let length_index = length as usize;
        let data_bytes : &[u8] = match val[8..length_index + 8].try_into(){
            Ok(val) => val,
            Err(_) => return error
        };
        let crc_data : &[u8] = match val[4..length_index + 8].try_into(){
            Ok(val) => val,
            Err(_) => return error 
        };
        let crc_bytes : [u8; 4] = match val[length_index + 8.. length_index + 12].try_into(){
            Ok(val) => val,
            Err(_) => return error
        };
        let crc = u32::from_be_bytes(crc_bytes);
        let crc_check = checksum_ieee(crc_data);
        if crc != crc_check{
            return error
        }
        let chunk_data : Vec<u8> = data_bytes.to_vec();
        let result = Chunk{
                          length,
            chunk_type,
            crc,
            chunk_data
        };
        Ok(result)
    }
} 
impl Display for Chunk{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{

 
      write!(f, "length -> {} \n", self.length())?;
        write!(f, "chunk type -> {} \n", self.chunk_type())?;
        write!(f, "crc -> {}", self.crc)?;
        write!(f, "data -> {:?}", self.chunk_data)
    }
}

impl Chunk{
    fn length(&self) -> u32{
        self.length
    }

    fn chunk_type(&self) -> &ChunkType{
        &self.chunk_type
    }

    fn data(&self) -> &[u8]{
        &self.chunk_data
    }

    fn crc(&self) -> u32{
        self.crc
    }

    fn data_as_string(&self) -> Result<String, String>{
        if let Ok(result_st) = from_utf8(&self.data()){
            return Ok(String::from(result_st))
        } else{
            return Err(String::from("Parse error"))
        }
    }

    fn as_bytes(&self)-> Vec<u8>{
        self.data().iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}