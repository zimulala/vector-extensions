/// Standalone data generator for TopSQL and TopRU
/// Can be used to generate sample data for testing

use chrono::Utc;
use rand::Rng;
use rand::distr::Alphanumeric;
use vector_lib::event::{Event, LogEvent, Value as LogValue};

use crate::sources::topsql_v2::upstream::consts::{
    LABEL_DATE, LABEL_INSTANCE_KEY, LABEL_KEYSPACE_NAME, LABEL_PLAN_DIGEST,
    LABEL_SQL_DIGEST, LABEL_SOURCE_TABLE, LABEL_TIMESTAMPS, LABEL_USER,
    METRIC_NAME_CPU_TIME_MS, METRIC_NAME_EXEC_DURATION, METRIC_NAME_NETWORK_IN_BYTES,
    METRIC_NAME_NETWORK_OUT_BYTES, METRIC_NAME_STMT_DURATION_COUNT,
    METRIC_NAME_STMT_DURATION_SUM_NS, METRIC_NAME_STMT_EXEC_COUNT, METRIC_NAME_TOTAL_RU,
    SOURCE_TABLE_TIDB_TOPRU, SOURCE_TABLE_TIDB_TOPSQL,
};

/// Generate random hex string for digest
fn generate_random_digest() -> String {
    let mut rng = rand::rng();
    (0..64)
        .map(|_| format!("{:x}", rng.random::<u8>() % 16))
        .collect()
}

/// Generate random keyspace name
fn generate_random_keyspace() -> String {
    let prefixes = ["keyspace_prod", "keyspace_test", "keyspace_dev"];
    let mut rng = rand::rng();
    let idx: usize = (rng.random::<u64>() % prefixes.len() as u64) as usize;
    let prefix = prefixes[idx];
    format!("{}_{:03}", prefix, rng.random::<u32>() % 1000)
}

/// Generate random user name
fn generate_random_user() -> String {
    let prefixes = ["app_user", "test_user", "admin_user", "service_user"];
    let mut rng = rand::rng();
    let idx: usize = (rng.random::<u64>() % prefixes.len() as u64) as usize;
    let prefix = prefixes[idx];
    format!("{}_{:03}", prefix, rng.random::<u32>() % 1000)
}

/// Generate N TopSQL records
pub fn generate_topsql_records(count: usize, instance_index: usize) -> Vec<Event> {
    let mut events = Vec::with_capacity(count);
    let mut rng = rand::rng();
    
    let now = Utc::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let instance_key = format!("topsql_tidb_127.0.1.{}", instance_index);
    
    for i in 0..count {
        let timestamp = now.timestamp() + i as i64;
        let mut event = Event::Log(LogEvent::default());
        let log = event.as_mut_log();
        
        log.insert(LABEL_SOURCE_TABLE, SOURCE_TABLE_TIDB_TOPSQL);
        log.insert(LABEL_TIMESTAMPS, LogValue::from(timestamp));
        log.insert(LABEL_DATE, LogValue::from(date_str.clone()));
        log.insert(LABEL_INSTANCE_KEY, instance_key.clone());
        log.insert(LABEL_SQL_DIGEST, generate_random_digest());
        log.insert(LABEL_PLAN_DIGEST, generate_random_digest());
        log.insert(METRIC_NAME_CPU_TIME_MS, LogValue::from((rng.random::<u32>() % 1000) + 10));
        log.insert(METRIC_NAME_STMT_EXEC_COUNT, LogValue::from((rng.random::<u64>() % 1000) + 1));
        log.insert(METRIC_NAME_STMT_DURATION_SUM_NS, LogValue::from((rng.random::<u64>() % 50000000) + 1000000));
        log.insert(METRIC_NAME_STMT_DURATION_COUNT, LogValue::from((rng.random::<u64>() % 1000) + 1));
        log.insert(METRIC_NAME_NETWORK_IN_BYTES, LogValue::from((rng.random::<u64>() % 1000000) + 10000));
        log.insert(METRIC_NAME_NETWORK_OUT_BYTES, LogValue::from((rng.random::<u64>() % 2000000) + 20000));
        
        events.push(event);
    }
    
    events
}

/// Generate N TopRU records
pub fn generate_topru_records(count: usize, instance_index: usize) -> Vec<Event> {
    let mut events = Vec::with_capacity(count);
    let mut rng = rand::rng();
    
    let now = Utc::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let instance_key = format!("topru_tidb_127.0.1.{}", instance_index);
    
    for i in 0..count {
        let timestamp = now.timestamp() + (i as i64 * 60); // Every minute
        let mut event = Event::Log(LogEvent::default());
        let log = event.as_mut_log();
        
        log.insert(LABEL_SOURCE_TABLE, SOURCE_TABLE_TIDB_TOPRU);
        log.insert(LABEL_TIMESTAMPS, LogValue::from(timestamp));
        log.insert(LABEL_DATE, LogValue::from(date_str.clone()));
        log.insert(LABEL_INSTANCE_KEY, instance_key.clone());
        log.insert(LABEL_KEYSPACE_NAME, generate_random_keyspace());
        log.insert(LABEL_USER, generate_random_user());
        log.insert(LABEL_SQL_DIGEST, generate_random_digest());
        log.insert(LABEL_PLAN_DIGEST, generate_random_digest());
        
        // Generate random RU consumption (100 - 10000)
        let total_ru = (rng.random::<u32>() % 9900) + 100;
        log.insert(METRIC_NAME_TOTAL_RU, LogValue::from(total_ru as f64 + (rng.random::<u32>() % 100) as f64 / 100.0));
        
        // Generate random execution count (100 - 10000)
        log.insert(METRIC_NAME_STMT_EXEC_COUNT, LogValue::from((rng.random::<u64>() % 9900) + 100));
        
        // Generate random execution duration (10ms - 5000ms in nanoseconds)
        log.insert(METRIC_NAME_EXEC_DURATION, LogValue::from((rng.random::<u64>() % 4990000000) + 10000000));
        
        events.push(event);
    }
    
    events
}

/// Print event in human-readable format
pub fn print_event_simple(event: &Event, index: usize) {
    if let Event::Log(log) = event {
        println!("Record #{}", index + 1);
        
        // Get all fields
        if let Some(fields) = log.all_event_fields() {
            for (key, value) in fields {
                let formatted_value = match value {
                    LogValue::Bytes(b) => String::from_utf8_lossy(b).to_string(),
                    LogValue::Integer(i) => i.to_string(),
                    LogValue::Float(f) => format!("{:.2}", f),
                    LogValue::Boolean(b) => b.to_string(),
                    LogValue::Timestamp(t) => t.to_string(),
                    _ => format!("{:?}", value),
                };
                println!("  {}: {}", key, formatted_value);
            }
        }
        println!();
    }
}

/// Print event in compact format
pub fn print_event_compact(event: &Event) {
    if let Event::Log(log) = event {
        if let Some(fields) = log.all_event_fields() {
            let mut parts = Vec::new();
            
            for (key, value) in fields {
                parts.push(format!("{}={:?}", key, value));
            }
            
            println!("{}", parts.join(", "));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_10_topsql_records() {
        println!("\n========== Generating 10 TopSQL Records ==========\n");
        let events = generate_topsql_records(10, 1);
        assert_eq!(events.len(), 10);
        
        for (i, event) in events.iter().enumerate() {
            print_event_simple(event, i);
        }
    }

    #[test]
    fn test_generate_10_topru_records() {
        println!("\n========== Generating 10 TopRU Records ==========\n");
        let events = generate_topru_records(10, 1);
        assert_eq!(events.len(), 10);
        
        for (i, event) in events.iter().enumerate() {
            print_event_simple(event, i);
        }
    }

    #[test]
    fn test_compare_topsql_vs_topru() {
        println!("\n========== TopSQL Record Example ==========\n");
        let topsql = generate_topsql_records(1, 1);
        print_event_simple(&topsql[0], 0);
        
        println!("\n========== TopRU Record Example ==========\n");
        let topru = generate_topru_records(1, 1);
        print_event_simple(&topru[0], 0);
    }

    #[test]
    fn test_generate_3_days_volume() {
        println!("\n========== 3 Days Data Volume Comparison ==========\n");
        
        // TopSQL: 1 record per second for 3 days
        let topsql_per_second = 1;
        let seconds_in_3_days = 3 * 24 * 60 * 60; // 259,200
        let topsql_total = topsql_per_second * seconds_in_3_days;
        
        // TopRU: 1 record per minute for 3 days
        let topru_per_minute = 1;
        let minutes_in_3_days = 3 * 24 * 60; // 4,320
        let topru_total = topru_per_minute * minutes_in_3_days;
        
        println!("TopSQL (per instance):");
        println!("  Frequency: {} record/second", topsql_per_second);
        println!("  Duration: 3 days = {} seconds", seconds_in_3_days);
        println!("  Total records: {} records/instance", topsql_total);
        println!();
        
        println!("TopRU:");
        println!("  Frequency: {} record/minute", topru_per_minute);
        println!("  Duration: 3 days = {} minutes", minutes_in_3_days);
        println!("  Total records: {} records", topru_total);
        println!();
        
        println!("Ratio: TopSQL has {}x more records than TopRU", topsql_total / topru_total);
    }
}
