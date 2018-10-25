#[cfg(not(feature = "pwm"))]
#[path = "nop.rs"]
mod write;

#[cfg(feature = "pwm")]
#[path = "pwm.rs"]
mod write;
