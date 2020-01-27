use std::convert::TryInto;
use toy_rsa_lib::*;

/* Fixed RSA encryption exponent.
E = 65537 */
pub const EXP: u64 = 65_537;

/* 𝜆(p, q):
return lcm(p - 1, q - 1) */
pub fn lambda(p: u64, q: u64) -> u64 {
    lcm(p - 1, q - 1)
}

/* Generate a pair of primes in the range `2**30..2**31`
suitable for RSA encryption with exponent
`EXP`. Warning: this routine has unbounded runtime; it
works by generate-and-test, generating pairs of primes
`p` `q` and testing that they satisfy `λ(pq) <= EXP` and
that `λ(pq)` has no common factors with `EXP`.

genkey:
    repeat
        p, q ← rsa primes (primes in range 230 .. 231)
    until E < 𝜆(p, q) and gcd(E, 𝜆) = 1
    return p, q */
pub fn genkey() -> (u32, u32) {
    let mut p = rsa_prime().try_into().unwrap();
    let mut q = rsa_prime().try_into().unwrap();

    while EXP > lambda(p, q) && gcd(EXP, lambda(p, q)) != 1 {
        print!("hit!");
        p = rsa_prime().try_into().unwrap();
        q = rsa_prime().try_into().unwrap();
    }

    // print!("p: {}, q: {}", p, q);
    (p.try_into().unwrap(), q.try_into().unwrap())
}

/* Encrypt the plaintext `msg` using the RSA public `key`
and return the ciphertext.

    encrypt(key, msg):
        return msgE mod key */
pub fn encrypt(key: u64, msg: u32) -> u64 {
    let msg: u64 = msg.try_into().unwrap();
    modexp(msg, EXP, key)
}

/* Decrypt the ciphertext `msg` using the RSA private `key`
and return the resulting plaintext.

    decrypt(key = p ⋅ q, msg):
        d ← inverse of 𝜆(p, q) mod E
        return msg**d mod (p ⋅ q)  */
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let (p, q) = key;
    let p: u64 = p.try_into().unwrap();
    let q: u64 = q.try_into().unwrap();
    let n = p * q;

    let d = modinverse(EXP, lambda(p, q));
    let result = modexp(msg, d, n);
    // print!("p: {}, q: {}, n: {}, result: {}\n", p, q, n, result);

    result.try_into().unwrap()
}
