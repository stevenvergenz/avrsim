use crate::register16::{Register16, Register16Mut};
use crate::status_flag::{StatusFlag, StatusFlagMut};

pub struct CpuState {
	// status registers, packed into a single byte
	pub status: u8,

	// general purpose registers
	pub reg: [u8; 32],
}

impl CpuState {
	pub fn c(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x01)
	}

	pub fn c_mut(&mut self) -> StatusFlagMut {
		StatusFlagMut::new(&mut self.status, 0x01)
	}

	pub fn x16(&self) -> Register16 {
		Register16::new(&self.reg[26..28])
	}

	pub fn x16_mut(&mut self) -> Register16Mut {
		Register16Mut::new(&mut self.reg[26..28])
	}

	pub fn y16(&self) -> Register16 {
		Register16::new(&self.reg[28..30])
	}

	pub fn y16_mut(&mut self) -> Register16Mut {
		Register16Mut::new(&mut self.reg[28..30])
	}

	pub fn z16(&self) -> Register16 {
		Register16::new(&self.reg[30..32])
	}

	pub fn z16_mut(&mut self) -> Register16Mut {
		Register16Mut::new(&mut self.reg[30..32])
	}
}
