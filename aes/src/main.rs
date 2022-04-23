use aes::cipher::generic_array::{typenum::U16, GenericArray};
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

fn bytes_to_u128(bytes: &[u8]) -> u128 {
    let mut arr: [u8; 16] = [0; 16];
    arr.copy_from_slice(bytes);
    u128::from_be_bytes(arr)
}

fn cbc_decrypt_block(aes: &Aes128, prev_block: &[u8], curr_block: &[u8]) -> Vec<u8> {
    let prev: [GenericArray<_, U16>; 1] = [*GenericArray::from_slice(prev_block); 1];
    let mut buf = [GenericArray::clone_from_slice(curr_block); 1];

    aes.decrypt_blocks(&mut buf);

    buf.get(0)
        .unwrap()
        .iter()
        .zip(prev.get(0).unwrap().iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn unpad(last_block: Vec<u8>) -> Vec<u8> {
    let last = *last_block.iter().last().unwrap();
    let pad_size = match last {
        1..=16 => {
            let pad_size: usize = last_block.iter().map(|byte| (*byte == last) as usize).sum();
            pad_size
        }
        _ => unreachable!(),
    };

    let last_block = match pad_size == last as usize {
        true => last_block.split_at(16 - pad_size).0.to_vec(),
        false => last_block,
    };
    last_block
}

fn cbc_decrypt(key: &[u8], blocks: Vec<u8>) -> Vec<u8> {
    let key: GenericArray<_, U16> = *GenericArray::from_slice(key);

    let aes = Aes128::new(&key);

    let padded: Vec<u8> = blocks
        .chunks(16)
        .collect::<Vec<_>>()
        .windows(2)
        .flat_map(|pair| cbc_decrypt_block(&aes, pair[0], pair[1]))
        .collect::<Vec<_>>();

    let mut blocks = padded.chunks(16).collect::<Vec<&[u8]>>();
    let last_block = blocks.pop().unwrap();
    let last_block = unpad(last_block.to_vec());
    blocks.push(&last_block);
    let message = blocks.iter().fold(vec![], |mut msg, block| {
        msg.extend(block.to_vec());
        msg
    });

    println!(
        "decoded: {:?}\nwithout_padding: {:?}",
        String::from_utf8_lossy(&padded),
        String::from_utf8_lossy(&message)
    );
    message
}

fn ctr_decrypt(key: Vec<u8>, blocks: Vec<u8>) -> Vec<u8> {
    let key: GenericArray<_, U16> = *GenericArray::from_slice(&key);
    // println!("blocks: {:?}", blocks.len());
    let iv = bytes_to_u128(blocks.split_at(16).0);
    let padded: Vec<u8> = blocks.split_at(16).1.to_vec();
    //
    // println!("padded: {:?}", padded.chunks(16).len());
    let v = padded.chunks(16).map(|c| c.to_vec());

    let vv = v
        .into_iter()
        .enumerate()
        .flat_map(|(i, block)| {
            let aes = Aes128::new(&key);
            let mut ctr_blk = (iv + i as u128).to_be_bytes();
            let mut buf = GenericArray::from_mut_slice(&mut ctr_blk);
            aes.encrypt_block(buf);
            buf.into_iter()
                .zip(block)
                .map(|(a, b)| *a ^ b)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    println!("decoded: {:?}", String::from_utf8_lossy(&vv));
    vv
}

fn main() {
    let key: Vec<u8> = hex::decode("140b41b22a29beb4061bda66b6747e14").unwrap();
    let code: Vec<u8> = hex::decode("4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a7897ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81").unwrap();

    cbc_decrypt(&key, code);

    let key: Vec<u8> = hex::decode("140b41b22a29beb4061bda66b6747e14").unwrap();
    let code: Vec<u8> = hex::decode("5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253")
        .unwrap();

    cbc_decrypt(&key, code);

    let key: Vec<u8> = hex::decode("36f18357be4dbd77f050515c73fcf9f2").unwrap();
    let code: Vec<u8> = hex::decode("69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb5054dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329")
        .unwrap();

    ctr_decrypt(key, code);

    let key: Vec<u8> = hex::decode("36f18357be4dbd77f050515c73fcf9f2").unwrap();
    let code: Vec<u8> = hex::decode("770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4e01726d3184c34451")
        .unwrap();

    ctr_decrypt(key, code);
}
