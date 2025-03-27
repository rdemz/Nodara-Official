// tests/unit_tests/nodara_standards_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_standards::pallet::{Pallet as StandardsPallet};

#[test]
fn test_standard_definition_and_verification() {
    new_test_ext().execute_with(|| {
        let id = b"Standard1".to_vec();
        let description = b"Initial Description".to_vec();
        let parameters = b"Params".to_vec();
        // Define a new standard.
        assert_ok!(StandardsPallet::<Test>::define_standard(RawOrigin::Signed(1).into(), id.clone(), description, parameters));
        // Verify compliance: simulate an operation that contains the parameter.
        let operation_data = b"Operation containing Params".to_vec();
        let result = StandardsPallet::<Test>::verify_compliance(id.clone(), operation_data);
        assert!(result.is_ok());
        assert!(result.unwrap());
    });
}
