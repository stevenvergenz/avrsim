pub struct Register16<'a> {
	r: &'a [u8],
}

impl<'a> Register16<'a> {
	pub fn new(r: &[u8]) -> Register16 {
		Register16 {
			r,
		}
	}

	pub fn value(&self) -> u16 {
		(self.r[0] as u16) | ((self.r[1] as u16) << 8)
	}
}

pub struct Register16Mut<'a> {
	r: &'a mut [u8],
}

impl<'a> Register16Mut<'a> {
	pub fn new(r: &mut [u8]) -> Register16Mut {
		Register16Mut {
			r,
		}
	}

	pub fn value(&self) -> u16 {
		(self.r[0] as u16) | ((self.r[1] as u16) << 8)
	}

	pub fn set(&mut self, value: u16) {
		self.r[0] = (value & 0xff) as u8;
		self.r[1] = ((value >> 8) & 0xff) as u8;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn read_only() {
		let arr = [1u8, 2, 3, 4];
		let reg = Register16::new(&arr[1..3]);
		assert_eq!(reg.value(), 0x0302);
	}

	#[test]
	fn read_write() {
		let mut arr = [1u8, 2, 3, 4];
		let mut reg = Register16Mut::new(&mut arr[1..3]);
		assert_eq!(reg.value(), 0x0302);

		reg.set(0xded0);
		assert_eq!(reg.value(), 0xded0);
		assert_eq!(arr[1..3], [0xd0, 0xde]);
	}
}
