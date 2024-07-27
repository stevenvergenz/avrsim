pub struct StatusFlag<'a> {
	status: &'a u8,
	mask: u8,
}

impl<'a> StatusFlag<'a> {
	pub fn new(status: &u8, mask: u8) -> StatusFlag {
		StatusFlag {
			status,
			mask,
		}
	}

	pub fn is_set(&self) -> bool {
		*self.status & self.mask == self.mask
	}
}

pub struct StatusFlagMut<'a> {
	status: &'a mut u8,
	mask: u8,
}

impl<'a> StatusFlagMut<'a> {
	pub fn new(status: &mut u8, mask: u8) -> StatusFlagMut {
		StatusFlagMut {
			status,
			mask,
		}
	}

	pub fn is_set(&self) -> bool {
		*self.status & self.mask == self.mask
	}

	pub fn set_to(&mut self, value: bool) {
		if value {
			self.set()
		}
		else {
			self.unset()
		}
	}

	pub fn set(&mut self) {
		*self.status = (*self.status & !self.mask) | self.mask;
	}

	pub fn unset(&mut self) {
		*self.status &= !self.mask
	}

	pub fn flip(&mut self) {
		*self.status ^= self.mask;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn read_only() {
		// TODO
	}

	#[test]
	fn read_write() {
		// TODO
	}
}
