# Guia de deploy — Roteador de logs de alta compressão

> Versão **1.0.0** · Stack: **Rust, zstd, lz4, Tokio**

## 1. Objetivo

Este guia descreve como publicar o projeto em **homologação** e **produção**, incluindo variáveis de ambiente, health checks e procedimento de rollback.

## 2. Pré-requisitos de infraestrutura

Rust 1.75+.

## 3. Homologação (staging)

### 3.1 Preparação

```bash
git clone https://github.com/SrSatriano/high-compression-log-router.git
cd high-compression-log-router
cp .env.example .env   # quando existir
```

### 3.2 Instalação

```bash
cargo build --release
cargo run --release --bin log_router
cargo run --release --bin bench_compression
```

### 3.3 Validação

```bash
cargo test
```

Confirme health check (HTTP ou métrica) antes de promover.

## 4. Produção

| Item | Recomendação |
|------|----------------|
| **Segredos** | Secret manager (GitHub Actions secrets, Vault, cloud provider) |
| **Rede** | TLS terminado no reverse proxy ou ingress |
| **Dados** | Backup automático conforme RPO/RTO da equipe |
| **Logs** | Agregação centralizada (Loki, CloudWatch, ELK) |
| **Métricas** | Prometheus/Grafana ou equivalente |

### Variáveis de ambiente

Consulte a tabela no [README](../README.md#configuração). Em produção, rotacione chaves periodicamente.

## 5. Estratégias de deploy

| Estratégia | Quando usar |
|------------|-------------|
| **Rolling** | APIs stateless, zero downtime gradual |
| **Blue/green** | Validação completa antes de trocar tráfego |
| **Canary** | Risco alto; libera % pequena do tráfego |

## 6. Health checks

- **Liveness:** processo responde (não travado).
- **Readiness:** dependências (banco, fila, RPC) disponíveis.

Documente o endpoint ou comando no README do serviço.

## 7. Rollback

1. Identifique a tag estável anterior (ex.: `v1.0.0`).
2. Reverta o deploy para a imagem/binário dessa tag.
3. Execute `cargo test` ou smoke tests automatizados.
4. Reprocesse fila morta ou jobs falhos, se aplicável.
5. Abra post-mortem se houve impacto a usuários.

## 8. Checklist pré-go-live

- [ ] Testes CI verdes
- [ ] `.env` de produção fora do Git
- [ ] Backup configurado
- [ ] Alertas de erro e latência ativos
- [ ] Runbook [OPERATIONS.md](OPERATIONS.md) revisado pela equipe
