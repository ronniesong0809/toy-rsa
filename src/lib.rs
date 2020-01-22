use std::convert::TryInto;
use toy_rsa_lib::*;

/* Fixed RSA encryption exponent.
E = 65537 */
pub const EXP: u64 = 65_537;

/* 𝜆(p, q):
return lcm(p - 1, q - 1) */
pub fn lambda(p: u64, q: u64) -> u64 {
    // let p: u64 = p.try_into().unwrap();
    // let q: u64 = q.try_into().unwrap();
    let p = u64::from(p);
    let q = u64::from(q);
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
    let mut p = 0;
    let mut q = 0;
    let mut done = false;

    while !done {
        p = u64::from(rsa_prime());
        q = u64::from(rsa_prime());

        if EXP < lambda(p, q) && gcd(EXP, lambda(p, q)) == 1 {
            print!("p: {}, q: {}\n", p, q);
            done = true;
        }
    }
    // p = u32::from(p).unwrap();
    let p: u32 = p.try_into().unwrap();
    let q: u32 = q.try_into().unwrap();
    (p, q)
}

/* Encrypt the plaintext `msg` using the RSA public `key`
and return the ciphertext.

    encrypt(key, msg):
        return msgE mod key */
pub fn encrypt(key: u64, msg: u32) -> u64 {
    let msg: u64 = msg.try_into().unwrap();
    modexp(msg, key, EXP)
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

    let lambda = lambda(p, q);
    let d = modinverse(lambda, EXP);

    let result: u32 = modexp(msg, p * q, d).try_into().unwrap();
    result
}
