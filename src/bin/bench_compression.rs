use std::time::Instant;

fn sample_json_log() -> Vec<u8> {
    (0..1000)
        .map(|i| {
            format!(
                r#"{{"ts":"2026-01-01T00:00:{:02}Z","level":"INFO","msg":"order filled","id":{}}}"#,
                i % 60,
                i
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
        .into_bytes()
}

fn main() -> anyhow::Result<()> {
    let data = sample_json_log();
    for (name, compressed) in [
        ("lz4", log_router::compress::compress_lz4(&data)?),
        ("zstd-3", log_router::compress::compress_zstd(&data, 3)?),
        ("zstd-19", log_router::compress::compress_zstd(&data, 19)?),
    ] {
        let t0 = Instant::now();
        let _ = &compressed;
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        let ratio = data.len() as f64 / compressed.len() as f64;
        println!("{name}: ratio={ratio:.2}x size={} time_ms={ms:.2}", compressed.len());
    }
    Ok(())
}
