extern crate xmz_shift_register;
use xmz_shift_register::ShiftRegister;

/// ./client led set 1 => leds.set(1)
#[test]
fn test_set_led() {
    let leds = ShiftRegister::new_led();
    let command = "led set 1";
    assert!(leds.data == 0);
}
