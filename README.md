# toy-rsa

[![Build Status](https://travis-ci.com/ronniesong0809/toy-rsa.svg?token=ysuqwpSTd1nLYmpB7CY5&branch=examples)](https://travis-ci.com/ronniesong0809/toy-rsa)

Copyright (c) 2020 Ronnie Song

This is a Rust based rsa library crate that provides RSA key generation, encryption and decryption.

<br>

*keygen()* usage:
```
let (p, q) = keygen();
```

*encrypt()* usage:
```
let secret = encrypt(key, msg);
```

*decrypt()* usage:
```
msg = decrypt((p, q), secret);
```

## Run Example

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

The tests are placed in tests/rand.rs file that uses std *assert_eq!()*, *assert_ne!()*, and extern [`more_assert`]() *assert_gt!()* to test equality of the actual result and expected result of the *keygen()*, *encrypt()* and *decrypt()* functions in that file.

[![Build Status](https://travis-ci.com/ronniesong0809/toy-rsa.svg?token=ysuqwpSTd1nLYmpB7CY5&branch=examples)](https://travis-ci.com/ronniesong0809/toy-rsa)

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
