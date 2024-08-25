use super::*;

#[allow(clippy::upper_case_acronyms)]
pub struct IROM {
    user: Vec<u8>,
    user_base: u32,
    kernel: Vec<u8>,
    kernel_base: u32,
}

impl IROM {
    pub fn new(user: &[u8], user_base: u32, kernel: &[u8], kernel_base: u32) -> Self {
        Self {
            user: user.to_vec(),
            user_base,
            kernel: kernel.to_vec(),
            kernel_base,
        }
    }

    pub fn fetch(&self, addr: u32) -> Result<u32> {
        if self.user_base <= addr && addr < self.user_base + self.user.len() as u32 {
            let offset = (addr - self.user_base) as usize;
            let inst = (self.user[offset] as u32)
                | ((self.user[offset + 1] as u32) << 8)
                | ((self.user[offset + 2] as u32) << 16)
                | ((self.user[offset + 3] as u32) << 24);
            Ok(inst)
        } else if self.kernel_base <= addr && addr < self.kernel_base + self.kernel.len() as u32 {
            let offset = (addr - self.kernel_base) as usize;
            let inst = (self.kernel[offset] as u32)
                | ((self.kernel[offset + 1] as u32) << 8)
                | ((self.kernel[offset + 2] as u32) << 16)
                | ((self.kernel[offset + 3] as u32) << 24);
            Ok(inst)
        } else {
            Err(anyhow!("Invalid instruction address: 0x{:08x}", addr)).with_context(|| context!())
        }
    }
}
