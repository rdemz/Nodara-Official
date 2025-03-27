#![cfg_attr(not(feature = "std"), no_std)]
#![cfg(feature = "offchain-worker")]
#![recursion_limit = "1024"]

//! # Offchain Workers for Nodara BIOSPHÈRE QUANTIC - Legendary Edition
//!
//! This module demonstrates offchain worker functionality using Rayon for parallel processing. 
//! These offchain tasks are designed to handle non-deterministic, computationally intensive operations,
//! such as heavy cryptographic verifications or bulk data aggregation, without impacting the deterministic on-chain runtime.
//!
//! ## Key Features:
//! - **Parallel Processing:** Leverages Rayon to distribute heavy computations across multiple cores.
//! - **Efficient Data Aggregation:** Aggregates and processes large datasets quickly.
//! - **Seamless Integration:** Designed to feed processed results back to the on-chain system when necessary.
//!
//! ## Example Functions:
//! - `parallel_sum`: Computes the sum of a vector of u8 values in parallel.
//! - `parallel_data_aggregation`: Aggregates data from multiple sources concurrently.
//!
//! *Note: This module is compiled only when the "offchain-worker" feature is enabled, ensuring that offchain tasks remain separate from deterministic on-chain logic.*

use sp_std::vec::Vec;
use rayon::prelude::*;

/// Computes the parallel sum of a vector of u8 values.
///
/// # Parameters
/// - `data`: A vector of u8 values representing the data to sum.
///
/// # Returns
/// The sum of all values in the vector as a u64.
pub fn parallel_sum(data: Vec<u8>) -> u64 {
    data.par_iter().map(|&x| x as u64).sum()
}

/// Example function for parallel data aggregation.
///
/// This function simulates aggregating multiple datasets concurrently.
/// Each dataset is a vector of u32 values, and the function computes the sum for each dataset in parallel,
/// then returns the total aggregated sum.
///
/// # Parameters
/// - `datasets`: A vector of datasets (each dataset is a vector of u32 values).
///
/// # Returns
/// The aggregated sum of all datasets as a u64.
pub fn parallel_data_aggregation(datasets: Vec<Vec<u32>>) -> u64 {
    datasets.par_iter()
        .map(|dataset| dataset.iter().sum::<u32>() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum() {
        let data = vec![1, 2, 3, 4, 5];
        let sum = parallel_sum(data);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_parallel_data_aggregation() {
        let datasets = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        let total = parallel_data_aggregation(datasets);
        // Sum of each dataset: 6, 15, 24 => Total = 45
        assert_eq!(total, 45);
    }
}
