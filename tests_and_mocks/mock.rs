// mock.rs
// Legendary mock runtime for Nodara BIOSPHÈRE QUANTIC, simulating a full blockchain environment for testing.

use sp_core::H256;
use frame_support::{parameter_types, traits::Everything};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use frame_system as system;

// Define basic types for the test runtime.
pub type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = system::mocking::MockBlock<Test>;

// Construct the legendary test runtime by integrating all critical modules.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        NodaraBiosphere: nodara_biosphere::{Pallet, Call, Storage, Event<T>},
        NodaraStabilityGuard: nodara_stability_guard::{Pallet, Call, Storage, Event<T>},
        NodaraLiquidityFlow: nodara_liquidity_flow::{Pallet, Call, Storage, Event<T>},
        NodaraReserveFund: nodara_reserve_fund::{Pallet, Call, Storage, Event<T>},
        NodaraRewardEngine: nodara_reward_engine::{Pallet, Call, Storage, Event<T>},
        NodaraID: nodara_id::{Pallet, Call, Storage, Event<T>},
        NodaraMarketplace: nodara_marketplace::{Pallet, Call, Storage, Event<T>},
        NodaraIoTBridge: nodara_iot_bridge::{Pallet, Call, Storage, Event<T>},
        NodaraStandards: nodara_standards::{Pallet, Call, Storage, Event<T>},
        // ... Intégrer ici les autres modules si nécessaire.
    }
);

// Define parameters for the legendary test runtime.
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024 * 1024;
    pub const AvailableBlockRatio: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(75);
}

impl system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = system::mocking::Origin;
    type RuntimeCall = ();
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Index = u64;
    type BlockHashCount = BlockHashCount;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type RuntimeEvent = ();
    type Version = ();
    type PalletInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ();
}

// Implement dummy configurations for each module. For production-grade tests, these should be fully implemented.
impl nodara_biosphere::pallet::Config for Test {
    type UnixTime = ();
    type RuntimeEvent = ();
    type BaselineMultiplier = ();
    type MaxMultiplier = ();
    type MinMultiplier = ();
}

impl nodara_stability_guard::pallet::Config for Test {
    type RuntimeEvent = ();
    type BaselineStability = ();
    type MaxStability = ();
    type MinStability = ();
}

impl nodara_liquidity_flow::pallet::Config for Test {
    type RuntimeEvent = ();
    type BaselineLiquidity = ();
    type MaxLiquidity = ();
    type MinLiquidity = ();
}

impl nodara_reserve_fund::pallet::Config for Test {
    type RuntimeEvent = ();
    type BaselineReserve = ();
    type MaxReserve = ();
    type MinReserve = ();
}

impl nodara_reward_engine::pallet::Config for Test {
    type RuntimeEvent = ();
    type InitialRewardPool = ();
    type BaseReward = ();
    type ReputationMultiplier = ();
    type MinReward = ();
    type MaxReward = ();
}

impl nodara_id::pallet::Config for Test {
    type RuntimeEvent = ();
    type DefaultVerification = ();
    type MaxKycLength = ();
}

impl nodara_marketplace::pallet::Config for Test {
    type RuntimeEvent = ();
    type MaxAssetMetadataLength = ();
    type MaxOrderMetadataLength = ();
    type BaseTradeFee = ();
}

impl nodara_iot_bridge::pallet::Config for Test {
    type RuntimeEvent = ();
    type MaxPayloadLength = ();
    type BaseTimeout = ();
}

impl nodara_standards::pallet::Config for Test {
    type RuntimeEvent = ();
    type MaxStandardLength = ();
}
