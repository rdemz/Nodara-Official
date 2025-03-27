# Tests and Mocks for Nodara BIOSPHÈRE QUANTIC - Legendary Edition

This folder provides a comprehensive testing framework for Nodara BIOSPHÈRE QUANTIC, designed to a mythical standard. It includes:
- **Formalized Unit Tests:** Comprehensive unit tests with internal invariant checks that simulate formal verification.
- **Integration Tests:** End-to-end tests that simulate real-world scenarios across multiple modules.
- **Benchmarking Scripts:** Integrated benchmarks using Substrate's frame-benchmarking to monitor performance at a granular level.
- **Fuzz Testing:** Internal fuzzing strategies to ensure robust handling of unexpected inputs.
- **Mock Runtime:** A complete mock environment built using Substrate’s `construct_runtime!` macro, enabling isolated and reproducible testing of each module.

These tests and mocks ensure that each component of the Nodara project is rigorously validated, providing a rock-solid foundation for production deployment.

## How to Run

- **Unit Tests:**  
  Run `cargo test --all` to execute all unit tests with detailed output.
- **Integration Tests:**  
  Run `cargo test --test integration_tests` to execute integration scenarios.
- **Benchmarks:**  
  Enable the "runtime-benchmarks" feature and run `cargo bench` to collect performance metrics.
- **Fuzz Testing:**  
  Custom fuzzing scripts (located in the `fuzz` subdirectory) can be executed to simulate random input scenarios.

*This testing suite is intended for developers and auditors who demand the highest level of code reliability and performance.*
