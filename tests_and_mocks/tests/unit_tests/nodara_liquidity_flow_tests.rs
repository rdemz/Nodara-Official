// tests/unit_tests/nodara_liquidity_flow_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_liquidity_flow::pallet::{Pallet as LiquidityFlowPallet};

#[test]
fn test_liquidity_adjustment() {
    new_test_ext().execute_with(|| {
        assert_ok!(LiquidityFlowPallet::<Test>::initialize_liquidity());
        let baseline = LiquidityFlowPallet::<Test>::liquidity_level();
        assert_ok!(LiquidityFlowPallet::<Test>::update_liquidity(100));
        let new_level = LiquidityFlowPallet::<Test>::liquidity_level();
        // Expect new liquidity to be baseline plus adjustment.
        assert!(new_level > baseline);
    });
}
