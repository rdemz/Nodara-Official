// tests/unit_tests/nodara_biosphere_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_biosphere::pallet::{Pallet as BiospherePallet, BioPhase};

#[test]
fn test_initialize_state() {
    new_test_ext().execute_with(|| {
        assert_ok!(BiospherePallet::<Test>::initialize_growth());
        let multiplier = BiospherePallet::<Test>::growth_multiplier();
        // Expect the multiplier to equal the baseline value.
        assert_eq!(multiplier, Test::BaselineMultiplier::get());
    });
}
