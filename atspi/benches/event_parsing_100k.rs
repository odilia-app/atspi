use atspi::Event;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod bench_utils;
use bench_utils::{generate_n_messages_rnd, read_messages_from_file, write_messages_to_file};

const FILE_PATH: &str = "100_000_messages.bin";
const N: usize = 100_000;

pub fn criterion_benchmark(c: &mut Criterion) {
	if !std::path::Path::new(FILE_PATH).exists() {
		let random_messages = generate_n_messages_rnd(N);
		write_messages_to_file(random_messages, FILE_PATH);
	}

	let random_messages = read_messages_from_file(FILE_PATH);

	c.bench_function("100_000 Messages into Events", |b| {
		b.iter(|| {
			random_messages.iter().map(Clone::clone).for_each(|m| {
				Event::try_from(black_box(m)).unwrap();
			})
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
