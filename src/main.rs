extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;

// PIN: EHRPWM2B (P8_13)
const BB_PWM_CHIP: u32 = 6;
const BB_PWM_NUMBER: u32 = 1;
const DUR: u64 = 1;

/// Make an LED "breathe" by increasing and
/// decreasing the brightness
fn main() {
    enable_pwm();

    let pwm = Pwm::new(BB_PWM_CHIP, BB_PWM_NUMBER).unwrap(); // number depends on chip, etc.
    loop {
	println!("High");
        pwm.with_exported(|| {
            pwm.enable(true).unwrap();
            pwm.set_period_ns(20_000).unwrap();
            pwm.set_duty_cycle_ns(10000)
        }).unwrap();

        sleep(Duration::new(DUR, 0));

        println!("Low");
        pwm.with_exported(|| {
            pwm.enable(true).unwrap();
            pwm.set_period_ns(20_000).unwrap();
            pwm.set_duty_cycle_ns(1000)
        }).unwrap();

        sleep(Duration::new(DUR, 0));
    }
}

pub fn enable_pwm() {
    // Enable PWM Drivers
    Command::new("sh").
        arg("enable-pwm.sh")
        .output()
        .expect("Failed to enable pwm driver!");

}