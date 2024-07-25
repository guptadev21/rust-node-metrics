use sysinfo::{System, SystemExt};

pub async fn collect_memory_usage() -> f64 {
    let mut system = System::new();
    system.refresh_all();
    (system.used_memory() as f64 / system.total_memory() as f64) * 100.0
}
