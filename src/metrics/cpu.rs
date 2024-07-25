use sysinfo::{CpuExt, System, SystemExt};

pub async fn collect_cpu_usage() -> f64 {
    let mut system = System::new();
    system.refresh_all();
    system.global_cpu_info().cpu_usage() as f64
}
