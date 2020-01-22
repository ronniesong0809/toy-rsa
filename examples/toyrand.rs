extern crate toy_rsa;
use toy_rsa::*;

pub fn main() {
    // Private Key
    let p: u32 = 0xed23e6cd;
    let q: u32 = 0xf050a04d;

    // Public Key
    let sum: u64 = 0xde9c5816141c8ba9;

    // Message
    let msg: u32 = 12345;
    // Message
    let msg2: u64 = 12345;

    // Encrypted:
    let _encrypted: u64 = 0x164e44b86776d497;

    // Decrypted
    let _decrypted: u32 = 12345;

    let encoding = encrypt(sum, msg);
    print!("encoding to {}\n", encoding);

    let decoding = decrypt((p, q), msg2);
    print!("decoding to {}\n", decoding);

    let (y, z) = genkey();
    print!("generating to {}, {}\n", y, z);
}
