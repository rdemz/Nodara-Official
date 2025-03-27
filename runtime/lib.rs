#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "1024"]

//! # Nodara BIOSPHÈRE QUANTIC Runtime - Legendary Edition
//!
//! This runtime integrates all core modules (pallets) of Nodara BIOSPHÈRE QUANTIC, forming the cohesive blockchain network.
//! It brings together modules for adaptive state management, decentralized governance, security, identity, asset management,
//! cross-chain interoperability, and more, using Substrate's modular architecture.
//!
//! ## Key Components:
//! - **Global Types:** Definitions for AccountId, Balance, BlockNumber, etc.
//! - **Module Integration:** Uses the `construct_runtime!` macro to assemble all pallets.
//! - **Runtime Version:** Provides versioning information to ensure compatibility.
//!
//! This runtime is designed to operate at legendary performance and reliability, serving as the backbone of Nodara BIOSPHÈRE QUANTIC.

use sp_runtime::{
    create_runtime_str, generic, traits::{BlakeTwo256, Block as BlockT, IdentityLookup},
    RuntimeVersion,
};
use sp_core::OpaqueMetadata;
use frame_support::{construct_runtime, parameter_types, traits::KeyOwnerProofSystem};
use frame_system as system;

// Import global types from our node_primitives (assumed to be defined elsewhere)
pub use node_primitives::{AccountId, Balance, BlockNumber, Hash};

/// Opaque types used by the runtime.
pub mod opaque {
    use super::*;
    pub type Block = generic::Block<Header, RuntimeCall>;
    pub type Header = <Block as BlockT>::Header;
    pub type BlockId = generic::BlockId<Block>;
    pub type Signature = sp_runtime::MultiSignature;
}

/// Runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("nodara-biosphere-quantic"),
    impl_name: create_runtime_str!("nodara-biosphere-quantic"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    transaction_version: 1,
    apis: RUNTIME_API_VERSIONS,
};

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;
    pub const MaximumBlockWeight: u32 = 2 * 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024 * 1024;
    pub const AvailableBlockRatio: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(75);
}

// Construct the runtime by integrating all the modules.
construct_runtime!(
    pub enum Runtime where
        Block = generic::Block<Header, RuntimeCall>,
        NodeBlock = generic::Block<Header, RuntimeCall>,
        UncheckedExtrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCall, sp_runtime::MultiSignature, SignedExtra>,
    {
        // System module provides essential system functionalities.
        System: system::{Pallet, Call, Config, Storage, Event<T>},

        // Timestamp for block time.
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},

        // Core modules of Nodara BIOSPHÈRE QUANTIC:
        Biosphere: nodara_biosphere::{Pallet, Call, Storage, Event<T>},
        GrowthModel: nodara_growth_model::{Pallet, Call, Storage, Event<T>},
        StabilityGuard: nodara_stability_guard::{Pallet, Call, Storage, Event<T>},
        LiquidityFlow: nodara_liquidity_flow::{Pallet, Call, Storage, Event<T>},
        ReserveFund: nodara_reserve_fund::{Pallet, Call, Storage, Event<T>},
        RewardEngine: nodara_reward_engine::{Pallet, Call, Storage, Event<T>},
        ID: nodara_id::{Pallet, Call, Storage, Event<T>},
        Marketplace: nodara_marketplace::{Pallet, Call, Storage, Event<T>},
        IoTBridge: nodara_iot_bridge::{Pallet, Call, Storage, Event<T>},
        Interop: nodara_interop::{Pallet, Call, Storage, Event<T>},
        PredictiveGuard: nodara_predictive_guard::{Pallet, Call, Storage, Event<T>},
        Reputation: nodara_reputation::{Pallet, Call, Storage, Event<T>},
        Standards: nodara_standards::{Pallet, Call, Storage, Event<T>},
        POW: nodara_pow::{Pallet, Call, Storage, Event<T>},
        // Additional modules can be added here.
    }
);

/// Metadata for opaque types.
pub mod opaque {
    use super::*;
    pub type Block = generic::Block<Header, RuntimeCall>;
    pub type Header = <Block as BlockT>::Header;
    pub type BlockId = generic::BlockId<Block>;
    pub type Signature = sp_runtime::MultiSignature;
}

/// Opaque metadata.
pub const OPAQUE_METADATA: OpaqueMetadata = OpaqueMetadata::new(sp_runtime::OpaqueExtrinsic::default().encode());

/// Additional runtime configurations can be added here (e.g., implementation of pallet_timestamp::Config, etc.).
