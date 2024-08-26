use super::*;

#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
pub struct CPU {
    regs: [u32; 32],
    pc: u32,
    csrs: [u32; 4096],
    irom: IROM,
    dram: DRAM,
}

impl CPU {
    /// Create a new `Cpu` object.
    pub fn new(
        user: &[u8],
        user_base: u32,
        kernel: &[u8],
        kernel_base: u32,
        stack_base: u32,
        stack_size: u32,
    ) -> Self {
        let mut regs: [u32; 32] = [0; 32]; // 默认寄存存放的是 无符号
        regs[2] = stack_base.wrapping_add(stack_size); // sp
        let pc = user_base;
        let mut csrs = [0; 4096];
        csrs[MTVAL as usize] = kernel_base;
        let irom = IROM::new(user, user_base, kernel, kernel_base);
        let dram = DRAM::new(user, user_base, stack_base, stack_size);
        Self {
            regs,
            pc,
            csrs,
            irom,
            dram,
        }
    }

    /// Get an instruction from the dram.
    pub fn fetch(&self) -> Result<u32> {
        self.irom.fetch(self.pc)
    }

    pub fn pc_step(&mut self) {
        self.pc += 4;
    }

    pub fn dump(&self) {
        // 还没有执行
        self.dump_csrs();
        self.dump_registers();
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    /// Execute an instruction after decoding. Return true if an error happens, otherwise false.
    pub fn execute(&mut self, inst: u32) -> Result<()> {
        // Emulate that register x0 is hardwired with all bits equal to 0.
        self.regs[0] = 0;
        let inst = Instr::try_from(inst).with_context(|| context!())?;
        match inst {
            Instr::LUI(rd, imm) => {
                self.regs[rd as usize] = imm;
            }
            Instr::AUIPC(rd, imm) => {
                self.regs[rd as usize] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            Instr::JAL(rd, offset) => {
                self.regs[rd as usize] = self.pc;
                self.pc = self.pc.wrapping_add(offset as u32).wrapping_sub(4);
            }
            Instr::BEQ(rs1, rs2, offset) => {
                if self.regs[rs1 as usize] == self.regs[rs2 as usize] {
                    self.pc = self.pc.wrapping_add(offset as u32).wrapping_sub(4);
                }
            }
            Instr::BNE(rs1, rs2, offset) => {
                if self.regs[rs1 as usize] != self.regs[rs2 as usize] {
                    self.pc = self.pc.wrapping_add(offset as u32).wrapping_sub(4);
                }
            }
            Instr::BLT(rs1, rs2, offset) => {
                if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) {
                    self.pc = self.pc.wrapping_add(offset as u32).wrapping_sub(4);
                }
            }
            Instr::BGE(rs1, rs2, offset) => {
                if (self.regs[rs1 as usize] as i32) >= (self.regs[rs2 as usize] as i32) {
                    self.pc = self.pc.wrapping_add(offset as u32).wrapping_sub(4);
                }
            }
            Instr::BLTU(rs1, rs2, offset) => {
                if self.regs[rs1 as usize] < self.regs[rs2 as usize] {
                    self.pc = self.pc.wrapping_add(offset as u32).wrapping_sub(4);
                }
            }
            Instr::BGEU(rs1, rs2, offset) => {
                if self.regs[rs1 as usize] >= self.regs[rs2 as usize] {
                    self.pc = self.pc.wrapping_add(offset as u32).wrapping_sub(4);
                }
            }
            Instr::SB(rs2, offset, rs1) => {
                let addr = self.regs[rs1 as usize].wrapping_add(offset as u32);
                self.store(addr, self.regs[rs2 as usize], 8)
                    .with_context(|| context!())?
            }
            Instr::SH(rs2, offset, rs1) => {
                let addr = self.regs[rs1 as usize].wrapping_add(offset as u32);
                self.store(addr, self.regs[rs2 as usize], 16)
                    .with_context(|| context!())?
            }
            Instr::SW(rs2, offset, rs1) => {
                let addr = self.regs[rs1 as usize].wrapping_add(offset as u32);
                self.store(addr, self.regs[rs2 as usize], 32)
                    .with_context(|| context!())?
            }
            Instr::ADDI(rd, rs1, imm) => {
                self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(imm as u32);
            }
            Instr::ANDI(rd, rs1, imm) => {
                self.regs[rd as usize] = self.regs[rs1 as usize] & (imm as u32);
            }
            Instr::ORI(rd, rs1, imm) => {
                self.regs[rd as usize] = self.regs[rs1 as usize] | (imm as u32);
            }
            Instr::XORI(rd, rs1, imm) => {
                self.regs[rd as usize] = self.regs[rs1 as usize] ^ (imm as u32);
            }
            Instr::SLLI(rd, rs1, imm) => {
                let shamt = (imm & 0x01f) as u32;
                self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shl(shamt);
            }
            Instr::SRLI(rd, rs1, imm) => {
                let shamt = (imm & 0x01f) as u32;
                self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shr(shamt);
            }
            Instr::SRAI(rd, rs1, imm) => {
                let shamt = (imm & 0x01f) as u32;
                self.regs[rd as usize] =
                    (self.regs[rs1 as usize] as i32).wrapping_shr(shamt) as u32;
            }
            Instr::SLTI(rd, rs1, imm) => {
                self.regs[rd as usize] = ((self.regs[rs1 as usize] as i32) < imm) as u32;
            }
            Instr::SLTIU(rd, rs1, imm) => {
                self.regs[rd as usize] = (self.regs[rs1 as usize] < (imm as u32)) as u32;
            }
            Instr::LB(rd, offset, base) => {
                let addr = self.regs[base as usize].wrapping_add(offset as u32);
                self.regs[rd as usize] =
                    self.load(addr, 8).with_context(|| context!())? as i8 as i32 as u32;
            }
            Instr::LH(rd, offset, base) => {
                let addr = self.regs[base as usize].wrapping_add(offset as u32);
                self.regs[rd as usize] =
                    self.load(addr, 16).with_context(|| context!())? as i16 as i32 as u32;
            }
            Instr::LW(rd, offset, base) => {
                let addr = self.regs[base as usize].wrapping_add(offset as u32);
                self.regs[rd as usize] = self.load(addr, 32).with_context(|| context!())?;
            }
            Instr::LBU(rd, offset, base) => {
                let addr = self.regs[base as usize].wrapping_add(offset as u32);
                self.regs[rd as usize] =
                    self.load(addr, 8).with_context(|| context!())? as u8 as u32;
            }
            Instr::LHU(rd, offset, base) => {
                let addr = self.regs[base as usize].wrapping_add(offset as u32);
                self.regs[rd as usize] =
                    self.load(addr, 16).with_context(|| context!())? as u16 as u32;
            }
            Instr::JALR(rd, offset, base) => {
                let t = self.pc;
                self.pc = (self.regs[base as usize].wrapping_add(offset as u32)) & !1;
                self.regs[rd as usize] = t;
            }
            Instr::ADD(rd, rs1, rs2) => {
                self.regs[rd as usize] =
                    self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);
            }
            Instr::SUB(rd, rs1, rs2) => {
                self.regs[rd as usize] =
                    self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);
            }
            Instr::SLL(rd, rs1, rs2) => {
                let shamt = self.regs[rs2 as usize] & 0x3f;
                self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shl(shamt);
            }
            Instr::SLT(rd, rs1, rs2) => {
                self.regs[rd as usize] =
                    ((self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32)) as u32;
            }
            Instr::XOR(rd, rs1, rs2) => {
                self.regs[rd as usize] = self.regs[rs1 as usize] ^ self.regs[rs2 as usize];
            }
            Instr::SRL(rd, rs1, rs2) => {
                let shamt = self.regs[rs2 as usize] & 0x3f;
                self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shr(shamt);
            }
            Instr::SRA(rd, rs1, rs2) => {
                let shamt = self.regs[rs2 as usize] & 0x3f;
                self.regs[rd as usize] =
                    (self.regs[rs1 as usize] as i32).wrapping_shr(shamt) as u32;
            }
            Instr::AND(rd, rs1, rs2) => {
                self.regs[rd as usize] = self.regs[rs1 as usize] & self.regs[rs2 as usize];
            }
            Instr::OR(rd, rs1, rs2) => {
                self.regs[rd as usize] = self.regs[rs1 as usize] | self.regs[rs2 as usize];
            }
            Instr::SLTU(rd, rs1, rs2) => {
                self.regs[rd as usize] = (self.regs[rs1 as usize] < self.regs[rs2 as usize]) as u32;
            }
            Instr::ECALL => {
                self.store_csr(MEPC, self.pc).with_context(|| context!())?;
                let mtval = self.load_csr(MTVAL).with_context(|| context!())?;
                self.pc = mtval;
                self.store_csr(MCAUSE, 0x0000_000b)
                    .with_context(|| context!())?;
            }
            Instr::ERET => {
                self.pc = self.load_csr(MEPC).with_context(|| context!())?;
                return Err(anyhow!("eret happened")).with_context(|| context!());
            }
            Instr::CSRRW(rd, csr_addr, rs1) => {
                let t = self.load_csr(csr_addr).with_context(|| context!())?;
                self.store_csr(csr_addr, self.regs[rs1 as usize])
                    .with_context(|| context!())?;
                self.regs[rd as usize] = t;
            }
            Instr::CSRRS(rd, csr_addr, rs1) => {
                let t = self.load_csr(csr_addr).with_context(|| context!())?;
                self.store_csr(csr_addr, t | self.regs[rs1 as usize])
                    .with_context(|| context!())?;
                self.regs[rd as usize] = t;
            }
            Instr::CSRRC(rd, csr_addr, rs1) => {
                let t = self.load_csr(csr_addr).with_context(|| context!())?;
                self.store_csr(csr_addr, t & !self.regs[rs1 as usize])
                    .with_context(|| context!())?;
                self.regs[rd as usize] = t;
            }
            Instr::CSRRWI(rd, csr_addr, zimm) => {
                self.regs[rd as usize] = self.load_csr(csr_addr).with_context(|| context!())?;
                self.store_csr(csr_addr, zimm).with_context(|| context!())?;
            }
            Instr::CSRRSI(rd, csr_addr, zimm) => {
                let t = self.load_csr(csr_addr).with_context(|| context!())?;
                self.store_csr(csr_addr, t | zimm)
                    .with_context(|| context!())?;
                self.regs[rd as usize] = t;
            }
            Instr::CSRRCI(rd, csr_addr, zimm) => {
                let t = self.load_csr(csr_addr).with_context(|| context!())?;
                self.store_csr(csr_addr, t & !zimm)
                    .with_context(|| context!())?;
                self.regs[rd as usize] = t;
            }
        }

        Ok(())
    }

    /// Load a value from a dram.
    fn load(&self, addr: u32, size: u32) -> Result<u32> {
        self.dram.load(addr, size)
    }

    /// Store a value to a dram.
    fn store(&mut self, addr: u32, value: u32, size: u32) -> Result<()> {
        self.dram.store(addr, value, size)
    }

    /// Store a value to a CSR.
    #[allow(dead_code)]
    fn store_csr(&mut self, addr: u32, value: u32) -> Result<()> {
        match addr {
            MSTATUS | MEPC | MCAUSE | MTVAL => {
                let addr = addr as usize;
                self.csrs[addr] = value;
            }
            _ => {
                return Err(anyhow!("not implemented yet: csr {:#x}", addr))
                    .with_context(|| context!());
            }
        }
        Ok(())
    }

    /// Load a value from a CSR.
    fn load_csr(&self, addr: u32) -> Result<u32> {
        match addr {
            MSTATUS | MEPC | MCAUSE | MTVAL => {
                let addr = addr as usize;
                Ok(self.csrs[addr])
            }
            _ => Err(anyhow!("not implemented yet: csr {:#x}", addr)).with_context(|| context!()),
        }
    }

    /// Print values in all registers (x0-x31).
    #[allow(clippy::format_in_format_args)]
    fn dump_registers(&self) {
        let mut output = String::from("");
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    " x{:02}({})={:#x}
                    x{:02}({})={:#x}
                    x{:02}({})={:#x}
                    x{:02}({})={:#x} ",
                    i,
                    ABI[i],
                    self.regs[i],
                    i + 1,
                    ABI[i + 1],
                    self.regs[i + 1],
                    i + 2,
                    ABI[i + 2],
                    self.regs[i + 2],
                    i + 3,
                    ABI[i + 3],
                    self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
        println!("pc = {:#x}", self.pc);
    }

    /// Print values in some csrs.
    fn dump_csrs(&self) {
        println!(
            "mstatus={:#x}\tmepc={:#x}\tmcause={:#x}",
            self.load_csr(MSTATUS).unwrap(),
            self.load_csr(MEPC).unwrap(),
            self.load_csr(MCAUSE).unwrap(),
        );
    }
}
