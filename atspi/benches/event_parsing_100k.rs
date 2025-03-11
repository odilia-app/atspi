use atspi::Event;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::time::Duration;

mod bench_utils;
use bench_utils::{generate_n_messages_rnd, read_messages_from_file, write_messages_to_file};

const FILE_PATH: &str = "100_000_messages.bin";
const N: usize = 100_000;

pub fn criterion_benchmark(c: &mut Criterion) {
	let mut c = c.benchmark_group("msg_parse");
	if !std::path::Path::new(FILE_PATH).exists() {
		let random_messages = generate_n_messages_rnd(N);
		write_messages_to_file(random_messages, FILE_PATH);
	}

	let random_messages = read_messages_from_file(FILE_PATH);

  c.sample_size(5_000);
	c.measurement_time(Duration::from_secs(30));
	c.throughput(Throughput::Elements(1));
	c.bench_function("100_000 Messages into Events", |b| {
    let mut idx = 0;
    let len = random_messages.len();
		b.iter_batched(
			|| {
        let input = &random_messages[idx % len];
        idx += 1;
        input
      },
			|msg: &zbus::Message| {
        black_box(Event::try_from(msg))
			},
			criterion::BatchSize::PerIteration
		);
	});
	c.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
