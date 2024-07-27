use crate::register16::{Register16, Register16Mut};
use crate::status_flag::{StatusFlag, StatusFlagMut};

/// Manage the various registers of the CPU
pub struct CpuState {
	/// Status registers, packed into a single byte
	pub status: u8,

	/// General purpose registers
	pub reg: [u8; 32],

	/// The stack pointer to data SRAM
	pub stack: u16,
}

impl CpuState {
	/// Read the carry flag
	pub fn c(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x01)
	}

	/// Write the carry flag
	pub fn c_mut(&mut self) -> StatusFlagMut {
		StatusFlagMut::new(&mut self.status, 0x01)
	}

	/// Read the zero flag
	pub fn z(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x02)
	}

	/// Write the zero flag
	pub fn z_mut(&mut self) -> StatusFlagMut {
		StatusFlagMut::new(&mut self.status, 0x02)
	}

	/// Read the negative flag
	pub fn n(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x04)
	}

	/// Write the negative flag
	pub fn n_mut(&mut self) -> StatusFlagMut {
		StatusFlagMut::new(&mut self.status, 0x04)
	}

	/// Read the twos-compliment overflow flag
	pub fn v(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x08)
	}

	/// Write the twos-compliment overflow flag
	pub fn v_mut(&mut self) -> StatusFlagMut {
		StatusFlagMut::new(&mut self.status, 0x08)
	}

	/// Read the sign flag (always the xor of n and v)
	pub fn s(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x10)
	}

	/// Read the half-carry flag
	pub fn h(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x20)
	}

	/// Write the half-carry flag
	pub fn h_mut(&mut self) -> StatusFlagMut {
		StatusFlagMut::new(&mut self.status, 0x20)
	}

	/// Read the bit copy storage flag
	pub fn t(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x40)
	}

	/// Write the bit copy storage flag
	pub fn t_mut(&mut self) -> StatusFlagMut {
		StatusFlagMut::new(&mut self.status, 0x40)
	}

	/// Read the global interrupt enable flag
	pub fn i(&self) -> StatusFlag {
		StatusFlag::new(&self.status, 0x80)
	}

	/// Write the global interrupt enable flag
	pub fn i_mut(&mut self) -> StatusFlagMut {
		StatusFlagMut::new(&mut self.status, 0x80)
	}

	/// Read the x register
	pub fn x16(&self) -> Register16 {
		Register16::new(&self.reg[26..28])
	}

	/// Write the x address register
	pub fn x16_mut(&mut self) -> Register16Mut {
		Register16Mut::new(&mut self.reg[26..28])
	}

	/// Read the y address register
	pub fn y16(&self) -> Register16 {
		Register16::new(&self.reg[28..30])
	}

	/// Write the y address register
	pub fn y16_mut(&mut self) -> Register16Mut {
		Register16Mut::new(&mut self.reg[28..30])
	}

	/// Read the z address register
	pub fn z16(&self) -> Register16 {
		Register16::new(&self.reg[30..32])
	}

	/// Write the z address register
	pub fn z16_mut(&mut self) -> Register16Mut {
		Register16Mut::new(&mut self.reg[30..32])
	}
}
