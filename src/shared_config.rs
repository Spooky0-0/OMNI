/// Shared constants and configurations for the DSCE Platform (DEC, DCSE, RAFCE)

// Unique identifier for the shared memory mapping
pub const SHARED_MEM_FLINK: &str = "omnibus_dsce_shm_link";

// 1 GB shared memory segment
pub const SHM_SIZE_BYTES: usize = 1024 * 1024 * 1024;

// Magic numbers and buffer capacities
#[allow(dead_code)]
pub const RING_BUFFER_CAPACITY: usize = 1_000_000;

// Gateway circuit breaker states and memory layout are now in dsce-types
