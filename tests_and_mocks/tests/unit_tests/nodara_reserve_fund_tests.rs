// tests/unit_tests/nodara_reserve_fund_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_reserve_fund::pallet::{Pallet as ReserveFundPallet};

#[test]
fn test_reserve_fund_operations() {
    new_test_ext().execute_with(|| {
        // Initialize the reserve with the baseline value.
        assert_ok!(ReserveFundPallet::<Test>::initialize_reserve());
        let baseline = ReserveFundPallet::<Test>::reserve_balance();
        // Contribute funds.
        assert_ok!(ReserveFundPallet::<Test>::contribute(100, b"Test Contribution".to_vec()));
        let increased = ReserveFundPallet::<Test>::reserve_balance();
        assert!(increased > baseline);
        // Withdraw funds.
        assert_ok!(ReserveFundPallet::<Test>::withdraw(50, b"Test Withdrawal".to_vec()));
        let decreased = ReserveFundPallet::<Test>::reserve_balance();
        assert!(decreased < increased);
    });
}
