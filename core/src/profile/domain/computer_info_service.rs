use sysinfo::System;

pub struct ComputerInfo {
    pub os: String,
    pub os_version: String,
}

impl ComputerInfo {
    pub fn to_string(&self) -> String {
        format!("Operating System: {} {}", self.os, self.os_version)
    }
}

pub fn get_computer_info() -> ComputerInfo {
    ComputerInfo {
        os: System::name().unwrap_or("".to_string()),
        os_version: System::kernel_version().unwrap_or("".to_string()),
    }
}
