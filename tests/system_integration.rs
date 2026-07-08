#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_three_stage_ignition() {
        println!(">>> Starting OMNIBUS Integration Test...");

        // 1. Spawn DEC
        // Note: use .exe extension for Windows locally, or omit on Linux
        #[cfg(target_os = "windows")]
        let mut dec = Command::new("./target/release/deterministic-exchange-core.exe").spawn().expect("Failed to start DEC");
        #[cfg(not(target_os = "windows"))]
        let mut dec = Command::new("./target/release/deterministic-exchange-core").spawn().expect("Failed to start DEC");
        
        // 2. Spawn DCSE
        #[cfg(target_os = "windows")]
        let mut dcse = Command::new("./target/release/dcse.exe").spawn().expect("Failed to start DCSE");
        #[cfg(not(target_os = "windows"))]
        let mut dcse = Command::new("./target/release/dcse").spawn().expect("Failed to start DCSE");
        
        // 3. Spawn RAFCE
        #[cfg(target_os = "windows")]
        let mut rafce = Command::new("./target/release/rafce.exe").spawn().expect("Failed to start RAFCE");
        #[cfg(not(target_os = "windows"))]
        let mut rafce = Command::new("./target/release/rafce").spawn().expect("Failed to start RAFCE");

        // Allow processes to stabilize
        thread::sleep(Duration::from_millis(500));

        // Check process health
        assert!(dec.try_wait().unwrap().is_none(), "DEC crashed prematurely");
        assert!(dcse.try_wait().unwrap().is_none(), "DCSE crashed prematurely");
        assert!(rafce.try_wait().unwrap().is_none(), "RAFCE crashed prematurely");

        // Cleanup
        dec.kill().unwrap();
        dcse.kill().unwrap();
        rafce.kill().unwrap();
        
        println!(">>> Grand Integration Test Passed: Ignition Sequence Verified.");
    }
}
