# Motor de Roteamento de Logs de Alta Compressão

Coletor Rust que comprime streams JSON com Zstd e envia via gRPC para cold storage.

## Stack

- Rust, zstd, gRPC (tonic)

## Comparativo de compressão (JSON logs)

| Algoritmo | Ratio | Throughput |
|-----------|-------|------------|
| none | 1.0× | baseline |
| LZ4 | ~2.5× | muito rápido |
| Zstd L3 | ~4.2× | equilíbrio |
| Zstd L19 | ~5.8× | CPU alto |

Benchmark: `cargo run --release --bin bench_compression`

## Stress I/O

Teste de disco: `cargo run --bin stress_ingest -- --gb 10 --out /data/logs`

Documentação: [docs/COMPRESSION_BENCH.md](docs/COMPRESSION_BENCH.md) | [docs/IO_STRESS.md](docs/IO_STRESS.md)

## Run collector

```bash
cargo run --bin log_router -- --listen 0.0.0.0:50051 --zstd-level 3
```

## Estrutura

| Pasta | Função |
|-------|--------|
| `src/router/` | ingest + batch |
| `src/compress/` | zstd |
| `src/grpc/` | tonic server |
