// tests/unit_tests/nodara_iot_bridge_tests.rs

use crate::mock::{new_test_ext, Test};
use frame_support::{assert_ok};
use nodara_iot_bridge::pallet::{Pallet as IoTBridgePallet};

#[test]
fn test_iot_data_submission() {
    new_test_ext().execute_with(|| {
        let id = 1;
        let payload = b"IoT Payload".to_vec();
        let device_id = b"Device01".to_vec();
        let signature = b"ValidSignature".to_vec();
        assert_ok!(IoTBridgePallet::<Test>::submit_iot_data(id, payload, device_id, signature));
    });
}
