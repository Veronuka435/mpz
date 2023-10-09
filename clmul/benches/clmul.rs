use criterion::{black_box, criterion_group, criterion_main, Criterion};

use clmul::Clmul;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = ChaCha12Rng::seed_from_u64(0);
    let a: [u8; 16] = rng.gen();
    let b: [u8; 16] = rng.gen();
    let a = Clmul::new(&a);
    let b = Clmul::new(&b);

    c.bench_function("clmul", move |bench| {
        bench.iter(|| {
            black_box(a.clmul(b));
        });
    });

    c.bench_function("reduce", move |bench| {
        bench.iter(|| black_box(Clmul::reduce_gcm(a, b)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
