# Quick Start Guide

## Prerequisites

Make sure `protoc` is installed and in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
protoc --version  # Should show: libprotoc 28.3
```

If not installed, run:
```bash
brew install protobuf
```

## Generate Data

Navigate to the project directory:
```bash
cd /Users/xia/workspace/src/github.com/yibin87/vector-extensions
```

### Generate 10 TopRU Records

```bash
cargo test --lib sources::mocked_topsql::generate_data::tests::test_generate_10_topru_records -- --nocapture
```

**Output Sample:**
```
Record #1
  date: Bytes(b"2026-01-20")
  instance_key: Bytes(b"topru_tidb_127.0.1.1")
  keyspace_name: Bytes(b"keyspace_prod_338")
  user: Bytes(b"app_user_817")
  sql_digest: Bytes(b"3f1124810b5b...")
  plan_digest: Bytes(b"6fd1df4ce838...")
  topru_total_ru: Float(5932.14)
  topsql_stmt_exec_count: Integer(4799)
  topru_exec_duration: Integer(2648763761)
```

### Compare TopSQL vs TopRU (1 of each)

```bash
cargo test --lib sources::mocked_topsql::generate_data::tests::test_compare_topsql_vs_topru -- --nocapture
```

This shows the **key differences**:
- **TopSQL**: Has `cpu_time_ms`, `network_in/out_bytes`, `stmt_duration`
- **TopRU**: Has `keyspace_name`, `user`, `total_ru` (unique to TopRU)

### View 3-Day Data Volume

```bash
cargo test --lib sources::mocked_topsql::generate_data::tests::test_generate_3_days_volume -- --nocapture
```

Shows:
- **TopSQL**: 259,200 records per instance (1/second)
- **TopRU**: 4,320 records (1/minute)

## Key Differences

| Feature | TopSQL | TopRU |
|---------|--------|-------|
| **Frequency** | 1/second | 1/minute |
| **Purpose** | Performance monitoring | Resource billing |
| **Unique Fields** | `cpu_time_ms`, network IO | `keyspace_name`, `user`, `total_ru` |
| **Aggregation** | SQL + Plan | SQL + Plan + Keyspace + User |

## Permanent Setup

Add to your `~/.zshrc` to make protoc always available:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```
