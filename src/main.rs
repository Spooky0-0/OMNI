use shared_memory::*;
use std::process::{Command, Child};
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::thread;

mod shared_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting OMNIBUS Master Control Program...");

    // Stage 1: Memory Bootstrap
    println!("[Stage 1] Bootstrapping Shared Memory...");
    
    // We create the shared memory segment using the cross-platform shared_memory crate.
    // In production (Linux), you'd ideally enable huge pages if supported.
    let shmem_conf = ShmemConf::new()
        .size(shared_config::SHM_SIZE_BYTES)
        .flink(shared_config::SHARED_MEM_FLINK);

    // If you're on Linux, you could potentially inject map_hugetlb here or rely on OS config
    let shmem = match shmem_conf.clone().create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => {
            println!("Warning: Shared memory link already exists. Overwriting/re-opening...");
            shmem_conf.open()?
        },
        Err(e) => return Err(e.into()),
    };
    
    // Initialize the RingBufferHeader safely in the mapped memory
    let header_ptr = shmem.as_ptr() as *mut dsce_types::RingBufferHeader;
    unsafe {
        // Zero initialize the header area just in case
        std::ptr::write_bytes(header_ptr as *mut u8, 0, std::mem::size_of::<dsce_types::RingBufferHeader>());
        
        let header = &*header_ptr;
        header.system_state.store(dsce_types::SystemState::Starting as u8, Ordering::SeqCst);
    }
    
    println!("Shared memory successfully allocated ({} bytes) at flink '{}'", shared_config::SHM_SIZE_BYTES, shared_config::SHARED_MEM_FLINK);

    // Stage 2: Process Ignition
    println!("[Stage 2] Igniting Data Supply Chain Engines...");
    
    // NOTE: Paths assume binaries are built in target/release/ or target/debug/ of the workspace.
    // For this scaffold, we just specify the binary names. They will need to be in PATH or invoked from target/.
    
    // Launch RAFCE first
    let mut rafce_process = launch_engine("rafce")?;
    println!("RAFCE launched (PID: {})", rafce_process.id());
    
    // Launch DCSE second
    let mut dcse_process = launch_engine("dcse")?;
    println!("DCSE launched (PID: {})", dcse_process.id());
    
    // Launch DEC last
    let mut dec_process = launch_engine("deterministic-exchange-core")?;
    println!("DEC launched (PID: {})", dec_process.id());

    unsafe {
        let header = &*header_ptr;
        header.system_state.store(dsce_types::SystemState::Running as u8, Ordering::SeqCst);
    }

    // Stage 3: Health Monitoring
    println!("[Stage 3] Entering Health Monitoring Loop...");
    
    loop {
        // Monitor process status
        if let Some(status) = rafce_process.try_wait()? {
            println!("CRITICAL: RAFCE Engine Terminated ({:?}). Transitioning to Drain Only.", status);
            transition_to_drain_only(header_ptr);
            break;
        }
        
        if let Some(status) = dcse_process.try_wait()? {
            println!("CRITICAL: DCSE Engine Terminated ({:?}). Transitioning to Drain Only.", status);
            transition_to_drain_only(header_ptr);
            break;
        }
        
        if let Some(status) = dec_process.try_wait()? {
            println!("CRITICAL: DEC Engine Terminated ({:?}). Transitioning to Drain Only.", status);
            transition_to_drain_only(header_ptr);
            break;
        }

        // Add heartbeat checking logic here if necessary...
        
        thread::sleep(Duration::from_millis(500));
    }
    
    // Graceful Shutdown/Draining
    println!("Initiating Graceful Shutdown...");
    let _ = dec_process.kill();
    let _ = dcse_process.kill();
    let _ = rafce_process.kill();
    
    Ok(())
}

fn launch_engine(name: &str) -> Result<Child, std::io::Error> {
    // Use current_exe() to find where the orchestrator itself is running, 
    // then join the path to the other binaries in the same folder.
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove "omnibus.exe"

    #[cfg(target_os = "windows")]
    let binary_name = format!("{}.exe", name);
    #[cfg(not(target_os = "windows"))]
    let binary_name = name.to_string();

    let full_path = path.join(binary_name);

    Command::new(full_path)
        .env("OMNIBUS_SHM_FLINK", shared_config::SHARED_MEM_FLINK)
        .spawn()
}

fn transition_to_drain_only(header_ptr: *mut dsce_types::RingBufferHeader) {
    unsafe {
        let header = &*header_ptr;
        header.system_state.store(dsce_types::SystemState::DrainOnly as u8, Ordering::SeqCst);
    }
}
