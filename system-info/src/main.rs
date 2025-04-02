use deft::app::{App, IApp};
use deft::bootstrap;
use deft::js::js_engine::JsEngine;
use std::env;

use deft::{js_func, js_serialize};
use serde::Serialize;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysInfo {
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
    pub sys_name: Option<String>,
    pub sys_kernel_version: Option<String>,
    pub sys_os_version: Option<String>,
    pub sys_hostname: Option<String>,
    pub cpu_count: usize,
}

js_serialize!(SysInfo);

#[js_func]
pub async fn get_sys_info() -> SysInfo {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    sys.refresh_all();
    let mem_total = sys.total_memory();
    let mem_used = sys.used_memory();
    let mem_free = sys.free_memory();
    let sys_name = System::name();
    let sys_kernel_version = System::kernel_version();
    let sys_os_version = System::os_version();
    let sys_hostname = System::host_name();
    let cpu_count = sys.cpus().len();

    SysInfo {
        mem_total,
        mem_used,
        mem_free,
        sys_name,
        sys_kernel_version,
        sys_os_version,
        sys_hostname,
        cpu_count,
    }
}

struct MyApp {}

impl IApp for MyApp {
    fn init_js_engine(&mut self, js_engine: &mut JsEngine) {
        js_engine
            .eval_module(include_str!("../dev-hack.js"), "dev-hack.js")
            .unwrap();
        js_engine.add_global_func(get_sys_info::new());
    }

    fn create_module_loader(
        &mut self,
    ) -> Box<dyn deft::js::loader::JsModuleLoader + Send + Sync + 'static> {
        include!(concat!(env!("OUT_DIR"), "/js_loader.code"))
    }
}

fn main() {
    let app = App::new(MyApp {});
    bootstrap(app.clone());
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: deft::winit::platform::android::activity::AndroidApp) {
    let app = App::new(MyApp {});
    deft::android_bootstrap(android_app, app);
}
