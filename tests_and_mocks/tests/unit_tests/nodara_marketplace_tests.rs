// tests/unit_tests/nodara_marketplace_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_marketplace::pallet::{Pallet as MarketplacePallet, Order, OrderType, Trade};

#[test]
fn test_marketplace_flow() {
    new_test_ext().execute_with(|| {
        // Register an asset.
        assert_ok!(MarketplacePallet::<Test>::register_asset(RawOrigin::Signed(1).into(), 1, b"Asset Metadata".to_vec()));
        // Place a buy order.
        let buy_order = Order {
            id: 1,
            asset_id: 1,
            order_type: OrderType::Buy,
            price: 100,
            quantity: 10,
            account: 1,
            timestamp: MarketplacePallet::<Test>::current_timestamp(),
        };
        assert_ok!(MarketplacePallet::<Test>::place_order(RawOrigin::Signed(1).into(), buy_order.clone()));
        // Place a sell order.
        let sell_order = Order {
            id: 2,
            asset_id: 1,
            order_type: OrderType::Sell,
            price: 100,
            quantity: 10,
            account: 1,
            timestamp: MarketplacePallet::<Test>::current_timestamp(),
        };
        assert_ok!(MarketplacePallet::<Test>::place_order(RawOrigin::Signed(1).into(), sell_order.clone()));
        // Execute trade.
        let trade = Trade {
            id: 1,
            buy_order_id: 1,
            sell_order_id: 2,
            asset_id: 1,
            price: 100,
            quantity: 10,
            timestamp: MarketplacePallet::<Test>::current_timestamp(),
        };
        assert_ok!(MarketplacePallet::<Test>::execute_trade(RawOrigin::Signed(1).into(), trade));
    });
}
