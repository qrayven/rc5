# RC5 implementation

This library provides an implementation of the `RC5 32/12/16` cryptographic algorithm in Rust.

A reference whitepaper can be found here:
<https://people.csail.mit.edu/rivest/Rivest-rc5rev.pdf>

This is the naive first implementation and it's not optimized for performance and security. It supports both encryption and decryption and can be used to protect sensitive data.

For multi block-encryption it supports ECB block cipher mode, **which shouldn't be used in production**:

## Usage

```rust
    let key = [u8;16];
    let payload = vec![1u8, 1024];
    let rc = RC5::new(&[0_u8]).unwrap();

    let output = rc.encrypt_with_ecb(&payload);
```

## Testing

```bash
  cargo test
```

## Performance

```bash
  cargo bench
```
