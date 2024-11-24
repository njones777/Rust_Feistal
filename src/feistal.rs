use bit_vec::BitVec;


//Used to split X bit block size in order to performan feistal operation
pub fn split_block(block: &BitVec) -> (BitVec, BitVec){
    let block_size = block.len();
    println!("Block size: {}", block_size);
    let (mut block_1, mut block_2) = (BitVec::new(), BitVec::new());
    for i in 0..(block_size/2){
        block_1.push(block[i]);
        block_2.push(block[block_size / 2 + i]);
    }
    (block_1, block_2)
}


pub fn feistal_round(left_block: BitVec, mut right_block: BitVec, rounds: u8) -> BitVec {
    //If we are at our last round then flip the output and return the result
    if rounds == 0{
    println!("In function Right Block: {:?}", &right_block);
    println!("In function Left Block: {:?}", &left_block);
    let mut output = right_block.clone();
    output.extend(left_block.iter());
    return output;
    }
    //Clone right block
    let mut original_right_block = right_block.clone();

    //Apply function to right block (right now just flip the second bit)
    match right_block.get(1){
        Some(bit) => {
            if bit {
                //set to false
                right_block.set(1, false);
            } else {
                right_block.set(1, true);
            }
        } None => println!("Error on bit flip function"),
    }
    println!("XOR: Left({:?}), Right({:?})", &left_block, &right_block);
    
    //XOR the now function applied right side with left side and combine with original right side
    original_right_block.extend(xor_bitvecs(&right_block, &left_block));
    //original_right_block.extend(right_block);
    original_right_block

}

fn xor_bitvecs(bv1: &BitVec, bv2: &BitVec) -> BitVec {
    // Ensure the length of the result is the same as the shorter BitVec
    let len = bv1.len().min(bv2.len());  // Avoid out-of-bounds access
    let mut result = BitVec::with_capacity(len);

    for i in 0..len {
        // XOR the bits at the current index and push the result to the result BitVec
        result.push(bv1[i] ^ bv2[i]);
    }

    result
}