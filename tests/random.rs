extern crate toy_rsa;
use toy_rsa::*;
use toy_rsa_lib::*;

#[macro_use]
extern crate more_asserts;

#[test]
fn test_encrypt_hex() {
    let encrypted = encrypt(0xde9c5816141c8ba9, 12345);
    assert_eq!(encrypted, 0x164e44b86776d497);
}

#[test]
fn test_decrypt_hex() {
    let decrypted = decrypt((0xed23e6cd, 0xf050a04d), 0x164e44b86776d497);
    assert_eq!(decrypted, 12345);
}

#[test]
fn test_encrypt_decimal() {
    let encrypted = encrypt(5596146201337399507, 88888);
    assert_eq!(encrypted, 3444903676489968394);
}

#[test]
fn test_decrypt_decimal() {
    let decrypted = decrypt((2360746877, 2370498191), 3444903676489968394);
    assert_eq!(decrypted, 88888);
}

#[test]
fn test_less_then_exp() {
    let (p, q) = genkey();
    let result = lambda(p.into(), q.into());
    assert_gt!(result, 65_537);
}

#[test]
fn test_gcd_is_one() {
    let (p, q) = genkey();
    let result = gcd(EXP, lambda(p.into(), q.into()));
    assert_eq!(result, 1);
}

#[test]
fn test_genkey() {
    let (p, q) = genkey();
    let check = 65_537 < lambda(p.into(), q.into()) && gcd(EXP, lambda(p.into(), q.into())) == 1;
    assert_eq!(check, true);
}
