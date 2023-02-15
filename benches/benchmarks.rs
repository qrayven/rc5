use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rc5_test::Word;

fn encrypt_block_benchmark(c: &mut Criterion) {
    let key = vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    let pt: [Word; 2] = [0x01_11_22_33, 0x44_55_77_88];
    let mut ct: [Word; 2] = [0; 2];

    let rc = rc5_test::RC5::new(&key).expect("instance should be created");

    c.bench_function("encrypt 1  block, key: 16bytes", |b| {
        b.iter(|| rc.encrypt_block(black_box(&pt), &mut ct))
    });
}

fn decrypt_block_benchmark(c: &mut Criterion) {
    let key = vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    let pt: [Word; 2] = [0x01_11_22_33, 0x44_55_77_88];
    let mut ct: [Word; 2] = [0; 2];

    let rc = rc5_test::RC5::new(&key).expect("instance should be created");

    c.bench_function("decrypt 1 block, key: 16 bytes", |b| {
        b.iter(|| rc.decrypt_block(black_box(&pt), &mut ct))
    });
}

criterion_group!(benches, encrypt_block_benchmark, decrypt_block_benchmark);
criterion_main!(benches);
