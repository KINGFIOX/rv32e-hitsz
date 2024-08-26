use super::*;

#[allow(clippy::upper_case_acronyms)]
pub struct DRAM {
    /// 高地址
    stack: Vec<u8>,
    stack_base: u32,
    /// .data 段
    data: Vec<u8>,
    data_base: u32,
}

impl DRAM {
    fn align_up(x: u32, align: u32) -> u32 {
        (x + align - 1) / align * align
    }

    pub fn new(data: &[u8], data_base: u32, stack_base: u32, stack_size: u32) -> Self {
        let stack_size = DRAM::align_up(stack_size, 4);
        Self {
            stack: vec![0; stack_size as usize],
            stack_base,
            data: data.to_vec(),
            data_base,
        }
    }

    pub fn load(&self, addr: u32, size: u32) -> Result<u32> {
        if self.stack_base <= addr && addr < self.stack_base + self.stack.len() as u32 {
            let offset = (addr - self.stack_base) as usize;
            match size {
                8 => {
                    let data = self.stack[offset] as u32;
                    Ok(data)
                }
                16 => {
                    let data = (self.stack[offset] as u32) | ((self.stack[offset + 1] as u32) << 8);
                    Ok(data)
                }
                32 => {
                    let data = (self.stack[offset] as u32)
                        | ((self.stack[offset + 1] as u32) << 8)
                        | ((self.stack[offset + 2] as u32) << 16)
                        | ((self.stack[offset + 3] as u32) << 24);
                    Ok(data)
                }
                _ => Err(anyhow!("Invalid data size: {}", size)).with_context(|| context!()),
            }
        } else if self.data_base <= addr
            && (addr as usize) < (self.data_base as usize) + self.data.len()
        {
            let offset = (addr - self.data_base) as usize;
            match size {
                8 => {
                    let data = self.data[offset] as u32;
                    Ok(data)
                }
                16 => {
                    let data = (self.data[offset] as u32) | ((self.data[offset + 1] as u32) << 8);
                    Ok(data)
                }
                32 => {
                    let data = (self.data[offset] as u32)
                        | ((self.data[offset + 1] as u32) << 8)
                        | ((self.data[offset + 2] as u32) << 16)
                        | ((self.data[offset + 3] as u32) << 24);
                    Ok(data)
                }
                _ => Err(anyhow!("Invalid data size: {}", size)).with_context(|| context!()),
            }
        } else {
            Err(anyhow!("Invalid data address: 0x{:08x}", addr)).with_context(|| context!())
        }
    }

    pub fn store(&mut self, addr: u32, data: u32, size: u32) -> Result<()> {
        if self.stack_base <= addr && (addr as usize) < self.stack_base as usize + self.stack.len()
        {
            let offset = (addr - self.stack_base) as usize;
            match size {
                8 => {
                    self.stack[offset] = data as u8;
                    Ok(())
                }
                16 => {
                    self.stack[offset] = data as u8;
                    self.stack[offset + 1] = (data >> 8) as u8;
                    Ok(())
                }
                32 => {
                    self.stack[offset] = data as u8;
                    self.stack[offset + 1] = (data >> 8) as u8;
                    self.stack[offset + 2] = (data >> 16) as u8;
                    self.stack[offset + 3] = (data >> 24) as u8;
                    Ok(())
                }
                _ => Err(anyhow!("Invalid data size: {}", size)).with_context(|| context!()),
            }
        } else if self.data_base <= addr
            && (addr as usize) < self.data_base as usize + self.data.len()
        {
            let offset = (addr - self.data_base) as usize;
            match size {
                8 => {
                    self.data[offset] = data as u8;
                    Ok(())
                }
                16 => {
                    self.data[offset] = data as u8;
                    self.data[offset + 1] = (data >> 8) as u8;
                    Ok(())
                }
                32 => {
                    self.data[offset] = data as u8;
                    self.data[offset + 1] = (data >> 8) as u8;
                    self.data[offset + 2] = (data >> 16) as u8;
                    self.data[offset + 3] = (data >> 24) as u8;
                    Ok(())
                }
                _ => Err(anyhow!("Invalid data size: {}", size)).with_context(|| context!()),
            }
        } else {
            Err(anyhow!(
                "Invalid data address: 0x{:08x} <= 0x{:08x} < 0x{:08x}",
                self.stack_base,
                addr,
                self.stack_base + self.stack.len() as u32
            ))
            .with_context(|| context!())
        }
    }
}
