# Mocked TopSQL and TopRU Data Generator

This module generates mock data for TopSQL and TopRU metrics.

## Overview

- **TopSQL**: Collects SQL execution metrics **per second** from TiDB instances
- **TopRU**: Collects Resource Unit (RU) consumption metrics **per minute**, aggregated by keyspace and user

## Data Structures

### TopSQL Record

TopSQL records are generated every **1 second** per TiDB instance.

**Fields:**
- `source_table`: "tidb_topsql"
- `timestamps`: Unix timestamp in seconds
- `date`: Date string (YYYY-MM-DD)
- `instance_key`: TiDB instance identifier (e.g., "topsql_tidb_127.0.1.1")
- `sql_digest`: 64-character hex string identifying the SQL statement
- `plan_digest`: 64-character hex string identifying the execution plan
- `topsql_cpu_time_ms`: CPU time in milliseconds (u32)
- `topsql_stmt_exec_count`: Number of executions (i64)
- `topsql_stmt_duration_sum_ns`: Total execution duration in nanoseconds (i64)
- `topsql_stmt_duration_count`: Number of durations counted (i64)
- `topsql_network_in_bytes`: Network input in bytes (i64)
- `topsql_network_out_bytes`: Network output in bytes (i64)

**Example TopSQL Record 1:**
```
source_table: "tidb_topsql"
timestamps: 1737375483
date: "2026-01-20"
instance_key: "topsql_tidb_127.0.1.1"
sql_digest: "a1b2c3d4e5f6789012345678901234567890123456789012345678901234abcd"
plan_digest: "plan1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab"
topsql_cpu_time_ms: 150
topsql_stmt_exec_count: 25
topsql_stmt_duration_sum_ns: 5000000  (5ms total)
topsql_stmt_duration_count: 25
topsql_network_in_bytes: 102400  (100KB)
topsql_network_out_bytes: 204800  (200KB)
```

**Example TopSQL Record 2:**
```
source_table: "tidb_topsql"
timestamps: 1737375483
date: "2026-01-20"
instance_key: "topsql_tidb_127.0.1.2"
sql_digest: "deadbeef1234567890abcdef1234567890abcdef1234567890abcdef12345678"
plan_digest: "cafebabe7890abcdef1234567890abcdef1234567890abcdef1234567890abcd"
topsql_cpu_time_ms: 320
topsql_stmt_exec_count: 50
topsql_stmt_duration_sum_ns: 12000000  (12ms total)
topsql_stmt_duration_count: 50
topsql_network_in_bytes: 512000  (500KB)
topsql_network_out_bytes: 1024000  (1MB)
```

### TopRU Record

TopRU records are generated every **1 minute** (60 seconds), aggregated by:
- Keyspace name
- User name  
- SQL digest
- Plan digest

**Fields:**
- `source_table`: "tidb_topru"
- `timestamps`: Unix timestamp in seconds
- `date`: Date string (YYYY-MM-DD)
- `instance_key`: TiDB instance identifier (e.g., "topru_tidb_127.0.1.1")
- `keyspace_name`: Keyspace identifier (e.g., "keyspace_prod_001")
- `user`: User name (e.g., "app_user_001")
- `sql_digest`: 64-character hex string identifying the SQL statement
- `plan_digest`: 64-character hex string identifying the execution plan
- `topru_total_ru`: Total Resource Units consumed (f64)
- `topsql_stmt_exec_count`: Total number of executions in this minute (i64)
- `topru_exec_duration`: Total execution duration in nanoseconds (i64)

**Example TopRU Record 1:**
```
source_table: "tidb_topru"
timestamps: 1737375483
date: "2026-01-20"
instance_key: "topru_tidb_127.0.1.1"
keyspace_name: "keyspace_prod_001"
user: "app_user_001"
sql_digest: "a1b2c3d4e5f6789012345678901234567890123456789012345678901234abcd"
plan_digest: "plan1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab"
topru_total_ru: 1250.75  (Total RU in this minute)
topsql_stmt_exec_count: 1500  (Executions in this minute)
topru_exec_duration: 300000000  (300ms total duration)
```

**Example TopRU Record 2:**
```
source_table: "tidb_topru"
timestamps: 1737375483
date: "2026-01-20"
instance_key: "topru_tidb_127.0.1.2"
keyspace_name: "keyspace_test_002"
user: "test_user_042"
sql_digest: "deadbeef1234567890abcdef1234567890abcdef1234567890abcdef12345678"
plan_digest: "cafebabe7890abcdef1234567890abcdef1234567890abcdef1234567890abcd"
topru_total_ru: 3425.50  (Total RU in this minute)
topsql_stmt_exec_count: 3000  (Executions in this minute)
topru_exec_duration: 720000000  (720ms total duration)
```

## Data Volume Comparison

For **3 days** of data:

### TopSQL
- Frequency: **1 record per second**
- Duration: 3 days = 259,200 seconds
- **Total records: 259,200 records per TiDB instance**

### TopRU  
- Frequency: **1 record per minute**
- Duration: 3 days = 4,320 minutes
- **Total records: 4,320 records** (aggregated across all instances, per keyspace/user/sql/plan combination)

## TopRU Protocol Structure Reference

Based on the protobuf definition:

```rust
type TopRURecord struct {
    KeyspaceName []byte             // Keyspace identifier
    User         string             // User name
    SqlDigest    []byte             // SQL digest hash
    PlanDigest   []byte             // Plan digest hash
    Items        []*TopRURecordItem // Time series items
}

type TopRURecordItem struct {
    TimestampSec uint64  // Unix timestamp in seconds
    TotalRu      float64 // Total RU consumed
    ExecCount    uint64  // Number of executions
    ExecDuration uint64  // Total execution duration in nanoseconds
}
```

## Running Tests

To test the example generation:

```bash
cargo test --package vector-extensions --lib sources::mocked_topsql::examples
```

## Usage in Code

```rust
use crate::sources::mocked_topsql::examples::{
    generate_topsql_examples, 
    generate_topru_examples,
    print_event
};

// Generate 2 TopSQL records
let topsql_events = generate_topsql_examples();
for (i, event) in topsql_events.iter().enumerate() {
    print_event(event, &format!("TopSQL Record {}", i + 1));
}

// Generate 2 TopRU records
let topru_events = generate_topru_examples();
for (i, event) in topru_events.iter().enumerate() {
    print_event(event, &format!("TopRU Record {}", i + 1));
}
```
