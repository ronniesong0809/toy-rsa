extern crate toy_rsa;
use toy_rsa::*;

pub fn main() {
    // Private Key
    let (p, q) = genkey();
    // let p = 0xed23e6cd;
    // let q = 0xf050a04d;
    print!("\np: {}, q: {}\n", p, q);

    let d: u64 = p as u64 * q as u64;
    print!("public: {}\n", d);

    // Message
    let msg = 12345;

    let encrypted = encrypt(d, msg);
    print!("encrypted: {}\n", encrypted);

    let decrypted = decrypt((p, q), encrypted);
    print!("decrypted: {}\n", decrypted);
}