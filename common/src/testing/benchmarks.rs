use std::{future::Future, io, io::Write, thread, time::Instant};

pub async fn benchmark<F, Fut>(benchmark_name: &str, runs: usize, to_bench: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    let mut time = 0usize;
    let mut max = usize::MIN;
    let mut min = usize::MAX;

    // warm up
    thread::sleep(std::time::Duration::from_secs(2));

    for _ in 0..runs {
        let start_time = Instant::now();
        let _ = to_bench().await;
        let end_time = start_time.elapsed().as_millis() as usize;

        max = std::cmp::max(max, end_time);
        min = std::cmp::min(min, end_time);
        time += end_time;
    }

    let _ = io::stdout().write_all(
        format!(
            "  {}:\n    {}ms ± {}ms per run ( with {} runs ).\n\n",
            benchmark_name,
            time / runs,
            std::cmp::max(max - (time / runs), (time / runs) - min),
            runs
        )
        .as_bytes(),
    );
}
