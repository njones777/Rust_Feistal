use bit_vec::BitVec;
use std::fs::File;
use std::io::{Read, Write};
use Rust_Feistal::{cipher, feistal, file_utils};
//use file_utils::read_bytes_to_bitvec;

fn main() {
    let mut file = File::open("tests/small_test.txt").unwrap();
    let mut efile = File::create("tests/eouput.bin").unwrap();
    let mut refile = File::open("tests/eouput.bin").unwrap();
    let mut dfile = File::create("tests/doutput.txt").unwrap();
    let mut buffer = vec![0u8; (8 / 8) as usize];
    
    //Encryption loop
    loop {
        //attempt to read the number of bytes (block_size/8) into the buffer
        match file.read(&mut buffer){
            Ok(0) => {
                //EOF
                break;
            }
            Ok(_) => {
                //Convert byte(s) to a bit vec and apply feistal
                let plain_text_data = BitVec::from_bytes(&buffer);
                let mut cipher_text = plain_text_data.clone();

                //Feist rounds
                for i in (0..=1).rev(){
                    cipher_text = feistal_round(split_block(&cipher_text), i);
                    if i == 0{
                        //Convert cipher text BitVec back to byte array and right back to file
                        let byte_array_to_write = cipher_text.to_bytes();
                        efile.write_all(&byte_array_to_write).unwrap();
                    }

                }
                //Clear buffer for next read
                buffer.fill(0);
            }
            Err(e) => {
                eprintln!("Error while reading file: {}", e);
                break;
            }
        }
    }

    //Encryption loop
    loop {
        //attempt to read the number of bytes (block_size/8) into the buffer
        match refile.read(&mut buffer){
            Ok(0) => {
                //EOF
                break;
            }
            Ok(_) => {
                //Convert byte(s) to a bit vec and apply feistal
                let plain_text_data = BitVec::from_bytes(&buffer);
                let mut cipher_text = plain_text_data.clone();

                //Feist rounds
                for i in (0..=1).rev(){
                    cipher_text = feistal_round(split_block(&cipher_text), i);
                    if i == 0{
                        //Convert cipher text BitVec back to byte array and right back to file
                        let byte_array_to_write = cipher_text.to_bytes();
                        dfile.write_all(&byte_array_to_write).unwrap();
                    }

                }
                //Clear buffer for next read
                buffer.fill(0);
            }
            Err(e) => {
                eprintln!("Error while reading efile: {}", e);
                break;
            }
        }
    }

}
