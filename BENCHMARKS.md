# BENCHMARKS.md: Throughput & Latency

## Performance Methodology
Benchmarks are generated using `criterion` across 1,000,000-cycle iterations. All tests were executed on AMD Ryzen 5 5500 with Huge Page support enabled for the IPC ring buffer.

## Metrics
| Component | Metric | Result |
| :--- | :--- | :--- |
| **DEC** | P99 Latency | ~950ns |
| **RAFCE** | Ops/Sec | ~641M Ops/Sec |
| **DCSE** | Commit Throughput | ~2.6M TPS |

## Mechanical Sympathy
- **Memory:** Zero heap allocations occur in the hot-path. 
- **IPC:** The lock-free Disruptor buffer achieves 10-20ns handoff latency, effectively zero
