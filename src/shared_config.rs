/// Shared constants and configurations for the DSCE Platform (DEC, DCSE, RAFCE)

// Unique identifier for the shared memory mapping
pub const SHARED_MEM_FLINK: &str = "omnibus_dsce_shm_link";

// 1 GB shared memory segment
pub const SHM_SIZE_BYTES: usize = 1024 * 1024 * 1024;

// Magic numbers and buffer capacities
#[allow(dead_code)]
pub const RING_BUFFER_CAPACITY: usize = 1_000_000;

// Gateway circuit breaker states
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SystemState {
    Starting = 0,
    Running = 1,
    DrainOnly = 2,
    #[allow(dead_code)]
    Halted = 3,
}

// Memory layout definition
#[repr(C)]
pub struct RingBufferHeader {
    pub system_state: std::sync::atomic::AtomicU8,
    pub dec_heartbeat: std::sync::atomic::AtomicU64,
    pub dcse_heartbeat: std::sync::atomic::AtomicU64,
    pub rafce_heartbeat: std::sync::atomic::AtomicU64,
    // Other disruptor cursors and structures would follow...
}
