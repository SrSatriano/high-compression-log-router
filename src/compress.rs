pub fn compress_zstd(input: &[u8], level: i32) -> anyhow::Result<Vec<u8>> {
    Ok(zstd::encode_all(input, level)?)
}

pub fn compress_lz4(input: &[u8]) -> anyhow::Result<Vec<u8>> {
    Ok(lz4_flex::compress(input))
}
