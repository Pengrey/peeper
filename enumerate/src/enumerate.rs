use sysinfo::{System, Pid};
use regex::Regex;
use std::ffi::OsStr;

pub fn get_program_pid(program_name: &str, command_line_regex: Option<&str>) -> Option<Pid> {
    let regex_filter = if let Some(pattern) = command_line_regex {
        match Regex::new(pattern) {
            Ok(re) => Some(re),
            Err(_) => return None,
        }
    } else {
        None
    };

    let mut system = System::new_all();
    system.refresh_all();

    for process in system.processes_by_name(OsStr::new(program_name)) {
        if let Some(ref re) = regex_filter {
            let full_command = process.cmd().join(OsStr::new(" "));

            if let Some(cmd_str) = full_command.to_str() {
                if re.is_match(cmd_str) {
                    return Some(process.pid());
                }
            }
        } else {
            return Some(process.pid());
        }
    }

    None
}
