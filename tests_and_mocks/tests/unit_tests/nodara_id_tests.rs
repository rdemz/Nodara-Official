// tests/unit_tests/nodara_id_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_id::pallet::{Pallet as IdPallet};

#[test]
fn test_identity_registration_and_update() {
    new_test_ext().execute_with(|| {
        // Register identity.
        assert_ok!(IdPallet::<Test>::register_identity(RawOrigin::Signed(1).into(), b"Initial KYC".to_vec()));
        // Update identity.
        assert_ok!(IdPallet::<Test>::update_identity(RawOrigin::Signed(1).into(), b"Updated KYC".to_vec(), true));
    });
}
