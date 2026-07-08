# TESTS.md: Validation & Chaos Engineering

## 1. Unit Testing (Component Level)
All core modules (DEC matching logic, DCSE settlement state machine, RAFCE invariant checks) are tested via `cargo test`. We utilize property-based testing (via `proptest`) to generate millions of permutations, ensuring that edge cases—like volume overflows or wash-trading patterns—are caught at the contract level.

## 2. Chaos Engineering (System Level)
The `test_three_stage_ignition` integration test simulates the full lifecycle:
- **Crash Simulation:** We force-kill the RAFCE process while DEC is at 90% load to verify that the Gateway Circuit Breaker triggers and the Matching Core enters "Drain Only" mode without losing a single trade event.
- **Persistence Validation:** After killing the DCSE process, we verify the `wal_snapshot.bin` recovery by re-initializing the engine and confirming the total ledger balance matches the pre-crash state exactly.

## 3. Invariant Checks
- **Volumetric Consistency:** `verify_conservation_invariants` runs continuously to ensure `Total_Inbound == Total_Settled + Total_Rejected`.
