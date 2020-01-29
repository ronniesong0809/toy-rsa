# toy-rsa

[![Build Status](https://travis-ci.com/ronniesong0809/toy-rsa.svg?token=ysuqwpSTd1nLYmpB7CY5&branch=examples)](https://travis-ci.com/ronniesong0809/toy-rsa)

Copyright (c) 2020 Ronnie Song

This is a Rust based rsa library crate that provides RSA key generation, encryption and decryption.

```
generator -> pair of private keys
```
```
message + public key -> ciphertext 
```
```
ciphertext + private keys -> message
```

## Usages

#### Key Generation:
```
let (p, q) = keygen();
```
Generate a pair of primes in the range 2^30 ~2^31.

<br>

#### Encryption:
```
let secret = encrypt(key, msg);
```
Encrypt the `msg` using the RSA public key return the ciphertext.

<br>

#### Decryption:
```
msg = decrypt((p, q), secret);
```
Decrypt the `ciphertext` using the RSA pirvate key return the msg.

<br>

## Run Example
To run the example program, type the command below:
```
cargo run --example toyrand
```

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target\debug\examples\toyrand.exe`

msg: 12345
p: 3770801897, q: 4022411999
public: 15167718796344762103
encrypted: 10620660062450240214
decrypted: 12345
```
Everything went well! The input message and output message remain the same after been encoded and decoded.

## Test
To test the library crate, type the command below:
```
cargo test
```

```
    Finished test [unoptimized + debuginfo] target(s) in 0.23s
     Running target\debug\deps\random-cc8444ff128049ae.exe

running 7 tests
test test_encrypt_hex ... ok
test test_decrypt_hex ... ok
test test_encrypt_decimal ... ok
test test_decrypt_decimal ... ok
test test_genkey ... ok
test test_less_then_exp ... ok
test test_gcd_is_one ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All tests passed with no issues.

The tests are placed in tests/rand.rs file that uses std *assert_eq!()*, *assert_ne!()*, and extern [`more_assert`](https://docs.rs/more-asserts/0.2.1/more_asserts/) *assert_gt!()* to test equality of the actual result and expected result of the *keygen()*, *encrypt()* and *decrypt()* functions in that file.

Travis CI is running to do the automated testing. [![Build Status](https://travis-ci.com/ronniesong0809/toy-rsa.svg?token=ysuqwpSTd1nLYmpB7CY5&branch=examples)](https://travis-ci.com/ronniesong0809/toy-rsa)

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
