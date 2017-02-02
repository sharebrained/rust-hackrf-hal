
use lpc43xx::peripheral::gpio_port;

#[derive(Copy, Clone, PartialEq)]
pub struct GPIO {
	port: usize,
	pin: usize,
}

impl GPIO {
	pub fn set(&self) {
		gpio_port().set[self.port].write_word(1 << self.pin);
	}

	pub fn clear(&self) {
		gpio_port().clr[self.port].write_word(1 << self.pin);
	}

	pub fn toggle(&self) {
		gpio_port().not[self.port].write_word(1 << self.pin);
	}

	pub fn write(&self, value: u32) {
		gpio_port().w[self.port * 32 + self.pin].write_word(value);
	}
}

pub const LED_USB: GPIO = GPIO { port: 2, pin:  1 };
pub const LED_RX:  GPIO = GPIO { port: 2, pin:  2 };
pub const LED_TX:  GPIO = GPIO { port: 2, pin:  8 };

// Expansion GPIO.
pub const P2_0:    GPIO = GPIO { port: 5, pin:  0 };
pub const P2_1:    GPIO = GPIO { port: 5, pin:  1 };
pub const P2_3:    GPIO = GPIO { port: 5, pin:  3 };
pub const P2_8:    GPIO = GPIO { port: 5, pin:  7 };
pub const P2_4:    GPIO = GPIO { port: 5, pin:  4 };
pub const P2_9:    GPIO = GPIO { port: 1, pin: 10 };
pub const P2_13:   GPIO = GPIO { port: 1, pin: 13 };

pub const P1_8:    GPIO = GPIO { port: 1, pin:  1 };
pub const P1_5:    GPIO = GPIO { port: 1, pin:  8 };
pub const P6_1:    GPIO = GPIO { port: 3, pin:  0 };
pub const P6_2:    GPIO = GPIO { port: 3, pin:  1 };
