pub struct Register {
    // status registers, packed into a single byte
    pub status: u8,

    // general purpose registers
    pub r: [u8; 32],
}

impl Register {
    pub unsafe fn x<'a>(&'a self) -> &'a u16 {
        &*(self.r[26..27].as_ptr() as *const u16)
    }

    pub unsafe fn x_mut<'a>(&'a mut self) -> &'a mut u16 {
        &mut *(self.r[26..27].as_mut_ptr() as *mut u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x() {
        let mut r = Register {
            status: 0,
            r: [0; 32],
        };
        r.r[26] = 0x34;
        r.r[27] = 0x12;
        unsafe {
            assert_eq!(*r.x(), 0x1234);
        }

        unsafe {
            *r.x_mut() = 0x5678;
        }
        assert_eq!(r.r[26], 0x78);
        assert_eq!(r.r[27], 0x56);
    }
}