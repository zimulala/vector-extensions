# TopSQL & TopRU Data Generation Guide

## Running Tests to Generate Data

Change to the vector-extensions project directory:

```bash
cd /Users/xia/workspace/src/github.com/yibin87/vector-extensions
```

### 1. Generate 10 TopRU Records

```bash
cargo test --package vector-extensions --lib sources::mocked_topsql::generate_data::tests::test_generate_10_topru_records -- --nocapture
```

### 2. Generate 10 TopSQL Records

```bash
cargo test --package vector-extensions --lib sources::mocked_topsql::generate_data::tests::test_generate_10_topsql_records -- --nocapture
```

### 3. Compare TopSQL vs TopRU (Generate 1 of Each)

```bash
cargo test --package vector-extensions --lib sources::mocked_topsql::generate_data::tests::test_compare_topsql_vs_topru -- --nocapture
```

### 4. View 3-Day Data Volume Comparison

```bash
cargo test --package vector-extensions --lib sources::mocked_topsql::generate_data::tests::test_generate_3_days_volume -- --nocapture
```

### 5. Run All Tests

```bash
cargo test --package vector-extensions --lib sources::mocked_topsql::generate_data -- --nocapture
```

## Output Examples

### TopSQL Record Example

```
Record #1
  source_table: Bytes("tidb_topsql")
  timestamps: Integer(1737375483)
  date: Bytes("2026-01-20")
  instance_key: Bytes("topsql_tidb_127.0.1.1")
  sql_digest: Bytes("a3f2e1d4c5b6...")
  plan_digest: Bytes("8f7e6d5c4b3a...")
  topsql_cpu_time_ms: Integer(247)
  topsql_stmt_exec_count: Integer(523)
  topsql_stmt_duration_sum_ns: Integer(15234567)
  topsql_stmt_duration_count: Integer(523)
  topsql_network_in_bytes: Integer(245678)
  topsql_network_out_bytes: Integer(512345)
```

### TopRU Record Example

```
Record #1
  source_table: Bytes("tidb_topru")
  timestamps: Integer(1737375483)
  date: Bytes("2026-01-20")
  instance_key: Bytes("topru_tidb_127.0.1.1")
  keyspace_name: Bytes("keyspace_prod_042")
  user: Bytes("app_user_125")
  sql_digest: Bytes("d2e3f4a5b6c7...")
  plan_digest: Bytes("9a8b7c6d5e4f...")
  topru_total_ru: Float(2547.82)
  topsql_stmt_exec_count: Integer(3245)
  topru_exec_duration: Integer(875432109)
```

## Key Differences Comparison

| Feature | TopSQL | TopRU |
|---------|--------|-------|
| **Data Source** | `tidb_topsql` | `tidb_topru` |
| **Collection Frequency** | 1 record/second | 1 record/minute |
| **Instance Identifier** | `topsql_tidb_*` | `topru_tidb_*` |
| **Unique Fields** | CPU time, Network IO | Keyspace, User, RU consumption |
| **Purpose** | Performance monitoring | Resource billing, Quota management |
| **Aggregation Dimension** | SQL + Plan | SQL + Plan + Keyspace + User |
| **3-Day Data Volume** | 259,200 records/instance | 4,320 records |

## Usage in Code

```rust
use crate::sources::mocked_topsql::generate_data::{
    generate_topsql_records,
    generate_topru_records,
    print_event_simple,
};

// Generate 10 TopRU records
let topru_events = generate_topru_records(10, 1);
for (i, event) in topru_events.iter().enumerate() {
    print_event_simple(event, i);
}

// Generate 10 TopSQL records
let topsql_events = generate_topsql_records(10, 1);
for (i, event) in topsql_events.iter().enumerate() {
    print_event_simple(event, i);
}
```

## Generating Different Amounts of Data

Modify the `count` parameter in tests:

```rust
// Generate 100 TopRU records
let events = generate_topru_records(100, 1);

// Generate 1000 TopSQL records
let events = generate_topsql_records(1000, 1);

// Generate 3 days of TopRU data (4,320 records)
let events = generate_topru_records(4320, 1);

// Generate 3 days of TopSQL data (259,200 records)
let events = generate_topsql_records(259200, 1);
```

## Important Notes

1. The `--nocapture` flag is used to display `println!` output
2. The `instance_index` parameter is used to distinguish different TiDB instances
3. TopRU timestamps increment every minute (`timestamp + i * 60`)
4. TopSQL timestamps increment every second (`timestamp + i`)
