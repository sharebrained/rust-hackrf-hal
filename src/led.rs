
use gpio::{GPIO, LED_USB, LED_RX, LED_TX};

#[derive(Copy, Clone, PartialEq)]
pub struct LED {
	gpio: GPIO
}

impl LED {
	pub fn on(&self) {
		self.gpio.set();
	}

	pub fn off(&self) {
		self.gpio.clear();
	}

	pub fn toggle(&self) {
		self.gpio.toggle();
	}

	pub fn write(&self, value: u32) {
		self.gpio.write(value);
	}
}

pub const USB: LED = LED { gpio: LED_USB };
pub const RX:  LED = LED { gpio: LED_RX  };
pub const TX:  LED = LED { gpio: LED_TX  };
