// tests/unit_tests/nodara_stability_guard_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_stability_guard::pallet::{Pallet as StabilityGuardPallet};

#[test]
fn test_stability_update() {
    new_test_ext().execute_with(|| {
        assert_ok!(StabilityGuardPallet::<Test>::initialize_stability());
        let initial = StabilityGuardPallet::<Test>::stability_parameter();
        assert_ok!(StabilityGuardPallet::<Test>::update_stability(50));
        let updated = StabilityGuardPallet::<Test>::stability_parameter();
        // Check that the new stability parameter is greater than or equal to the initial value.
        assert!(updated >= initial);
    });
}
