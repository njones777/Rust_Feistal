use bit_vec::BitVec;
use std::io::{self, Read};
use std::fs::File;

//pub fn read_bytes_to_bitvec(block_size: u16, file_path: String) -> io::Result<BitVec> {
pub fn read_bytes_to_bitvec(block_size: u16, file_path: &str){
    let mut file = File::open(file_path).unwrap();
    let mut buffer = vec![0u8; (block_size / 8) as usize];
    println!("Start of Buffer: {:?}", &buffer);
    println!("Len of buffer: {}\n", buffer.len());


    loop {
        //attempt to read the number of bytes (block_size/8) into the buffer
        match file.read(&mut buffer){
            Ok(0) => {
                //EOF
                break;
            }
            Ok(bytes_read) => {
                //println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);
                //Clear buffer for next read
                buffer.fill(0);
            }
            Err(e) => {
                eprintln!("Error while reading file: {}", e);
                break;
            }
        }
    }
}