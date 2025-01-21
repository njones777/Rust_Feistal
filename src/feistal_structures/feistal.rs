use bit_vec::BitVec;


//Used to split X bit block size in order to performan feistal operation
pub fn split_block(block: &BitVec) -> (BitVec, BitVec){
    let block_size = block.len();
    let (mut block_1, mut block_2) = (BitVec::new(), BitVec::new());
    for i in 0..(block_size/2){
        block_1.push(block[i]);
        block_2.push(block[block_size / 2 + i]);
    }
    (block_1, block_2)
}

pub fn feistal_round(left_and_right_blocks: (BitVec, BitVec), rounds: u8) -> BitVec {
    //Deconstruct left and right block tuple
    let (left_block, mut right_block) = left_and_right_blocks;

    //If we are at our last round then flip the output and return the result
    if rounds == 0{
    let mut output = right_block.clone();
    output.extend(left_block.iter());
    return output;
    }
    //Clone right block
    let mut return_block = right_block.clone();

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
    //XOR the now function applied right side with left side and combine with original right side
    return_block.extend(xor_bitvecs(&right_block, &left_block));
    return_block
}

//Used for XOR operation portion of feistal structure
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