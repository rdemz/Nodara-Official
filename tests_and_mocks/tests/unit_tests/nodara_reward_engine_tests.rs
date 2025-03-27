// tests/unit_tests/nodara_reward_engine_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_reward_engine::pallet::{Pallet as RewardEnginePallet};

#[test]
fn test_reward_distribution() {
    new_test_ext().execute_with(|| {
        assert_ok!(RewardEnginePallet::<Test>::initialize_rewards());
        let initial_pool = RewardEnginePallet::<Test>::reward_pool();
        let account = 1;
        // Distribute reward based on a reputation score.
        assert_ok!(RewardEnginePallet::<Test>::distribute_reward(account, 50, b"Reward Test".to_vec()));
        let new_pool = RewardEnginePallet::<Test>::reward_pool();
        assert!(new_pool < initial_pool);
    });
}
