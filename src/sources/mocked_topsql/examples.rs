// Example: Generate 2 TopSQL records and 2 TopRU records
// This demonstrates the data structure for both types

use chrono::Utc;
use vector_lib::event::{Event, LogEvent, Value as LogValue};

// Import constants from the parent module
use crate::sources::topsql_v2::upstream::consts::{
    LABEL_DATE, LABEL_INSTANCE_KEY, LABEL_KEYSPACE_NAME, LABEL_PLAN_DIGEST,
    LABEL_SQL_DIGEST, LABEL_SOURCE_TABLE, LABEL_TIMESTAMPS, LABEL_USER,
    METRIC_NAME_CPU_TIME_MS, METRIC_NAME_EXEC_DURATION, METRIC_NAME_NETWORK_IN_BYTES,
    METRIC_NAME_NETWORK_OUT_BYTES, METRIC_NAME_STMT_DURATION_COUNT,
    METRIC_NAME_STMT_DURATION_SUM_NS, METRIC_NAME_STMT_EXEC_COUNT, METRIC_NAME_TOTAL_RU,
    SOURCE_TABLE_TIDB_TOPRU, SOURCE_TABLE_TIDB_TOPSQL,
};

/// Generate 2 TopSQL records
pub fn generate_topsql_examples() -> Vec<Event> {
    let now = Utc::now();
    let timestamp = now.timestamp();
    let date_str = now.format("%Y-%m-%d").to_string();
    
    let mut events = vec![];
    
    // TopSQL Record 1
    let mut event1 = Event::Log(LogEvent::default());
    let log1 = event1.as_mut_log();
    
    log1.insert(LABEL_SOURCE_TABLE, SOURCE_TABLE_TIDB_TOPSQL);
    log1.insert(LABEL_TIMESTAMPS, LogValue::from(timestamp));
    log1.insert(LABEL_DATE, LogValue::from(date_str.clone()));
    log1.insert(LABEL_INSTANCE_KEY, "topsql_tidb_127.0.1.1");
    log1.insert(LABEL_SQL_DIGEST, "a1b2c3d4e5f6789012345678901234567890123456789012345678901234abcd");
    log1.insert(LABEL_PLAN_DIGEST, "plan1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab");
    log1.insert(METRIC_NAME_CPU_TIME_MS, LogValue::from(150u32));
    log1.insert(METRIC_NAME_STMT_EXEC_COUNT, LogValue::from(25i64));
    log1.insert(METRIC_NAME_STMT_DURATION_SUM_NS, LogValue::from(5000000i64)); // 5ms total
    log1.insert(METRIC_NAME_STMT_DURATION_COUNT, LogValue::from(25i64));
    log1.insert(METRIC_NAME_NETWORK_IN_BYTES, LogValue::from(102400i64)); // 100KB
    log1.insert(METRIC_NAME_NETWORK_OUT_BYTES, LogValue::from(204800i64)); // 200KB
    
    events.push(event1);
    
    // TopSQL Record 2
    let mut event2 = Event::Log(LogEvent::default());
    let log2 = event2.as_mut_log();
    
    log2.insert(LABEL_SOURCE_TABLE, SOURCE_TABLE_TIDB_TOPSQL);
    log2.insert(LABEL_TIMESTAMPS, LogValue::from(timestamp));
    log2.insert(LABEL_DATE, LogValue::from(date_str.clone()));
    log2.insert(LABEL_INSTANCE_KEY, "topsql_tidb_127.0.1.2");
    log2.insert(LABEL_SQL_DIGEST, "deadbeef1234567890abcdef1234567890abcdef1234567890abcdef12345678");
    log2.insert(LABEL_PLAN_DIGEST, "cafebabe7890abcdef1234567890abcdef1234567890abcdef1234567890abcd");
    log2.insert(METRIC_NAME_CPU_TIME_MS, LogValue::from(320u32));
    log2.insert(METRIC_NAME_STMT_EXEC_COUNT, LogValue::from(50i64));
    log2.insert(METRIC_NAME_STMT_DURATION_SUM_NS, LogValue::from(12000000i64)); // 12ms total
    log2.insert(METRIC_NAME_STMT_DURATION_COUNT, LogValue::from(50i64));
    log2.insert(METRIC_NAME_NETWORK_IN_BYTES, LogValue::from(512000i64)); // 500KB
    log2.insert(METRIC_NAME_NETWORK_OUT_BYTES, LogValue::from(1024000i64)); // 1MB
    
    events.push(event2);
    
    events
}

/// Generate 2 TopRU records
/// TopRU is aggregated by keyspace, user, sql_digest, and plan_digest
/// Collected every minute (vs TopSQL every second)
pub fn generate_topru_examples() -> Vec<Event> {
    let now = Utc::now();
    let timestamp = now.timestamp();
    let date_str = now.format("%Y-%m-%d").to_string();
    
    let mut events = vec![];
    
    // TopRU Record 1
    let mut event1 = Event::Log(LogEvent::default());
    let log1 = event1.as_mut_log();
    
    log1.insert(LABEL_SOURCE_TABLE, SOURCE_TABLE_TIDB_TOPRU);
    log1.insert(LABEL_TIMESTAMPS, LogValue::from(timestamp));
    log1.insert(LABEL_DATE, LogValue::from(date_str.clone()));
    log1.insert(LABEL_INSTANCE_KEY, "topru_tidb_127.0.1.1");
    log1.insert(LABEL_KEYSPACE_NAME, "keyspace_prod_001");
    log1.insert(LABEL_USER, "app_user_001");
    log1.insert(LABEL_SQL_DIGEST, "a1b2c3d4e5f6789012345678901234567890123456789012345678901234abcd");
    log1.insert(LABEL_PLAN_DIGEST, "plan1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab");
    log1.insert(METRIC_NAME_TOTAL_RU, LogValue::from(1250.75)); // Total RU consumed in this minute
    log1.insert(METRIC_NAME_STMT_EXEC_COUNT, LogValue::from(1500i64)); // Executions in this minute
    log1.insert(METRIC_NAME_EXEC_DURATION, LogValue::from(300000000i64)); // 300ms total duration
    
    events.push(event1);
    
    // TopRU Record 2
    let mut event2 = Event::Log(LogEvent::default());
    let log2 = event2.as_mut_log();
    
    log2.insert(LABEL_SOURCE_TABLE, SOURCE_TABLE_TIDB_TOPRU);
    log2.insert(LABEL_TIMESTAMPS, LogValue::from(timestamp));
    log2.insert(LABEL_DATE, LogValue::from(date_str.clone()));
    log2.insert(LABEL_INSTANCE_KEY, "topru_tidb_127.0.1.2");
    log2.insert(LABEL_KEYSPACE_NAME, "keyspace_test_002");
    log2.insert(LABEL_USER, "test_user_042");
    log2.insert(LABEL_SQL_DIGEST, "deadbeef1234567890abcdef1234567890abcdef1234567890abcdef12345678");
    log2.insert(LABEL_PLAN_DIGEST, "cafebabe7890abcdef1234567890abcdef1234567890abcdef1234567890abcd");
    log2.insert(METRIC_NAME_TOTAL_RU, LogValue::from(3425.50)); // Total RU consumed in this minute
    log2.insert(METRIC_NAME_STMT_EXEC_COUNT, LogValue::from(3000i64)); // Executions in this minute
    log2.insert(METRIC_NAME_EXEC_DURATION, LogValue::from(720000000i64)); // 720ms total duration
    
    events.push(event2);
    
    events
}

/// Print event for debugging
pub fn print_event(event: &Event, label: &str) {
    println!("\n========== {} ==========", label);
    if let Event::Log(log) = event {
        if let Some(fields) = log.all_event_fields() {
            for (key, value) in fields {
                println!("{}: {:?}", key, value);
            }
        }
    }
    println!("====================================\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_topsql_examples() {
        let events = generate_topsql_examples();
        assert_eq!(events.len(), 2);
        
        for (i, event) in events.iter().enumerate() {
            print_event(event, &format!("TopSQL Record {}", i + 1));
        }
    }

    #[test]
    fn test_generate_topru_examples() {
        let events = generate_topru_examples();
        assert_eq!(events.len(), 2);
        
        for (i, event) in events.iter().enumerate() {
            print_event(event, &format!("TopRU Record {}", i + 1));
        }
    }
}
