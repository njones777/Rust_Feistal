use std::time::{SystemTime, UNIX_EPOCH};
fn main(){
    for i in 0..100{
        std::thread::sleep(std::time::Duration::from_millis(5));
        print!("{:08b}",((SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()%1000) % 256) ^ i as u128) ;
        // println!("test: {}", n);
        // println!("xor with 105: {}", n ^ x);
    }
}