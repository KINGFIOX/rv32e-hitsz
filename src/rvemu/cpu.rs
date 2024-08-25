use super::*;

const MSTATUS: u32 = 0x0300;
const MEPC: u32 = 0x0341;
const MCAUSE: u32 = 0x0342;

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
        dram_size: u32,
    ) -> Self {
        let mut regs = [0; 32];
        regs[2] = user_base + dram_size; // sp
        let pc = user_base;
        let csrs = [0; 4096];
        let irom = IROM::new(user, user_base, kernel, kernel_base);
        let dram = DRAM::new(user, user_base + dram_size, dram_size);
        Self {
            regs,
            pc,
            csrs,
            irom,
            dram,
        }
    }

    /// Print values in all registers (x0-x31).
    #[allow(clippy::format_in_format_args)]
    pub fn dump_registers(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                    i,
                    abi[i],
                    self.regs[i],
                    i + 1,
                    abi[i + 1],
                    self.regs[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.regs[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    /// Print values in some csrs.
    pub fn dump_csrs(&self) {
        println!(
            "mstatus={:>#18x} mepc={:>#18x} mcause={:>#18x}",
            self.load_csr(MSTATUS).unwrap(),
            self.load_csr(MEPC).unwrap(),
            self.load_csr(MCAUSE).unwrap(),
        );
    }

    /// Get an instruction from the dram.
    pub fn fetch(&self) -> Result<u32> {
        self.irom.fetch(self.pc)
    }

    pub fn pc_step(&mut self) {
        self.pc += 4;
    }

    /// Execute an instruction after decoding. Return true if an error happens, otherwise false.
    pub fn execute(&mut self, inst: u32) -> Result<()> {
        let opcode = inst & 0x0000007f;
        let rd = ((inst & 0x00000f80) >> 7) as usize;
        let rs1 = ((inst & 0x000f8000) >> 15) as usize;
        let rs2 = ((inst & 0x01f00000) >> 20) as usize;
        let funct3 = (inst & 0x00007000) >> 12;
        let funct7 = (inst & 0xfe000000) >> 25;

        // Emulate that register x0 is hardwired with all bits equal to 0.
        self.regs[0] = 0;

        match opcode {
            0x03 => {
                // imm[11:0] = inst[31:20]
                let imm = ((inst as i32) >> 20) as u32;
                let addr = self.regs[rs1].wrapping_add(imm);
                match funct3 {
                    0x0 => {
                        // lb
                        let val = self.load(addr, 8).with_context(|| context!())?;
                        self.regs[rd] = val as i8 as i32 as u32;
                    }
                    0x1 => {
                        // lh
                        let val = self.load(addr, 16).with_context(|| context!())?;
                        self.regs[rd] = val as i16 as i32 as u32;
                    }
                    0x2 => {
                        // lw
                        let val = self.load(addr, 32).with_context(|| context!())?;
                        self.regs[rd] = val;
                    }
                    0x4 => {
                        // lbu
                        let val = self.load(addr, 8).with_context(|| context!())?;
                        self.regs[rd] = val;
                    }
                    0x5 => {
                        // lhu
                        let val = self.load(addr, 16).with_context(|| context!())?;
                        self.regs[rd] = val;
                    }
                    _ => {
                        return Err(anyhow!(
                            "not implemented yet: opcode {:#x} funct3 {:#x}",
                            opcode,
                            funct3
                        ))
                        .with_context(|| context!());
                    }
                }
            }
            0x13 => {
                // imm[11:0] = inst[31:20]
                let imm = ((inst & 0xfff00000) as i32 >> 20) as u32;
                // "The shift amount is encoded in the lower 6 bits of the I-immediate field for RV64I."
                let shamt = imm & 0x3f;
                match funct3 {
                    0x0 => {
                        // addi
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                    }
                    0x1 => {
                        // slli
                        self.regs[rd] = self.regs[rs1] << shamt;
                    }
                    0x2 => {
                        // slti
                        self.regs[rd] = if (self.regs[rs1] as i64) < (imm as i64) {
                            1
                        } else {
                            0
                        };
                    }
                    0x3 => {
                        // sltiu
                        self.regs[rd] = if self.regs[rs1] < imm { 1 } else { 0 };
                    }
                    0x4 => {
                        // xori
                        self.regs[rd] = self.regs[rs1] ^ imm;
                    }
                    0x5 => {
                        match funct7 >> 1 {
                            // srli
                            0x00 => self.regs[rd] = self.regs[rs1].wrapping_shr(shamt),
                            // srai
                            0x10 => {
                                self.regs[rd] = (self.regs[rs1] as i32).wrapping_shr(shamt) as u32
                            }
                            _ => {}
                        }
                    }
                    0x6 => self.regs[rd] = self.regs[rs1] | imm, // ori
                    0x7 => self.regs[rd] = self.regs[rs1] & imm, // andi
                    _ => {}
                }
            }
            0x17 => {
                // auipc
                let imm = (inst & 0xfffff000) as i32 as u32;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            0x23 => {
                // imm[11:5|4:0] = inst[31:25|11:7]
                let imm = (((inst & 0xfe000000) as i32 >> 20) as u32) | ((inst >> 7) & 0x1f);
                let addr = self.regs[rs1].wrapping_add(imm);
                match funct3 {
                    0x0 => self.store(addr, 8, self.regs[rs2])?,  // sb
                    0x1 => self.store(addr, 16, self.regs[rs2])?, // sh
                    0x2 => self.store(addr, 32, self.regs[rs2])?, // sw
                    _ => {}
                }
            }
            0x33 => {
                let shamt = ((self.regs[rs2] & 0x3f) as u64) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        // add
                        self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                    }
                    (0x0, 0x01) => {
                        // mul
                        self.regs[rd] = self.regs[rs1].wrapping_mul(self.regs[rs2]);
                    }
                    (0x0, 0x20) => {
                        // sub
                        self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                    }
                    (0x1, 0x00) => {
                        // sll
                        self.regs[rd] = self.regs[rs1].wrapping_shl(shamt);
                    }
                    (0x2, 0x00) => {
                        // slt
                        self.regs[rd] = if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) {
                            1
                        } else {
                            0
                        };
                    }
                    (0x3, 0x00) => {
                        // sltu
                        self.regs[rd] = if self.regs[rs1] < self.regs[rs2] {
                            1
                        } else {
                            0
                        };
                    }
                    (0x4, 0x00) => {
                        // xor
                        self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
                    }
                    (0x5, 0x00) => {
                        // srl
                        self.regs[rd] = self.regs[rs1].wrapping_shr(shamt);
                    }
                    (0x5, 0x20) => {
                        // sra
                        self.regs[rd] = (self.regs[rs1] as i32).wrapping_shr(shamt) as u32;
                    }
                    (0x6, 0x00) => {
                        // or
                        self.regs[rd] = self.regs[rs1] | self.regs[rs2];
                    }
                    (0x7, 0x00) => {
                        // and
                        self.regs[rd] = self.regs[rs1] & self.regs[rs2];
                    }
                    _ => {
                        return Err(anyhow!(
                            "not implemented yet: opcode {:#x} funct3 {:#x} funct7 {:#x}",
                            opcode,
                            funct3,
                            funct7
                        ))
                        .with_context(|| context!());
                    }
                }
            }
            0x37 => {
                // lui
                self.regs[rd] = (inst & 0xfffff000) as i32 as u32;
            }
            0x63 => {
                // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
                let imm = (((inst & 0x80000000) as i32  >> 19) as u32)
                    | ((inst & 0x80) << 4) // imm[11]
                    | ((inst >> 20) & 0x7e0) // imm[10:5]
                    | ((inst >> 7) & 0x1e); // imm[4:1]

                match funct3 {
                    0x0 => {
                        // beq
                        if self.regs[rs1] == self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x1 => {
                        // bne
                        if self.regs[rs1] != self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x4 => {
                        // blt
                        if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x5 => {
                        // bge
                        if (self.regs[rs1] as i64) >= (self.regs[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x6 => {
                        // bltu
                        if self.regs[rs1] < self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x7 => {
                        // bgeu
                        if self.regs[rs1] >= self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    _ => {
                        return Err(anyhow!(
                            "not implemented yet: opcode {:#x} funct3 {:#x}",
                            opcode,
                            funct3
                        ))
                        .with_context(|| context!());
                    }
                }
            }
            0x67 => {
                // jalr
                let t = self.pc;
                let imm = (((inst & 0xfff00000) as i32) >> 20) as u32;
                self.pc = (self.regs[rs1].wrapping_add(imm)) & !1;
                self.regs[rd] = t;
            }
            0x6f => {
                // jal
                self.regs[rd] = self.pc;
                // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
                let imm = (((inst & 0x80000000) as i32  >> 11) as u32) // imm[20]
                    | (inst & 0xff000) // imm[19:12]
                    | ((inst >> 9) & 0x800) // imm[11]
                    | ((inst >> 20) & 0x7fe); // imm[10:1]
                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            0x73 => {
                let csr_addr = (inst & 0xfff00000) >> 20;
                match funct3 {
                    0x1 => {
                        // csrrw
                        let t = self.load_csr(csr_addr).with_context(|| context!())?;
                        self.store_csr(csr_addr, self.regs[rs1])
                            .with_context(|| context!())?;
                        self.regs[rd] = t;
                    }
                    0x2 => {
                        // csrrs
                        let t = self.load_csr(csr_addr).with_context(|| context!())?;
                        self.store_csr(csr_addr, t | self.regs[rs1])
                            .with_context(|| context!())?;
                        self.regs[rd] = t;
                    }
                    0x3 => {
                        // csrrc
                        let t = self.load_csr(csr_addr).with_context(|| context!())?;
                        self.store_csr(csr_addr, t & (!self.regs[rs1]))
                            .with_context(|| context!())?;
                        self.regs[rd] = t;
                    }
                    0x5 => {
                        // csrrwi
                        let zimm = rs1 as u32;
                        self.regs[rd] = self.load_csr(csr_addr).with_context(|| context!())?;
                        self.store_csr(csr_addr, zimm).with_context(|| context!())?;
                    }
                    0x6 => {
                        // csrrsi
                        let zimm = rs1 as u32;
                        let t = self.load_csr(csr_addr).with_context(|| context!())?;
                        self.store_csr(csr_addr, t | zimm)
                            .with_context(|| context!())?;
                        self.regs[rd] = t;
                    }
                    0x7 => {
                        // csrrci
                        let zimm = rs1 as u32;
                        let t = self.load_csr(csr_addr).with_context(|| context!())?;
                        self.store_csr(csr_addr, t & (!zimm))
                            .with_context(|| context!())?;
                        self.regs[rd] = t;
                    }
                    _ => {
                        return Err(anyhow!(
                            "not implemented yet: opcode {:#x} funct3 {:#x}",
                            opcode,
                            funct3
                        ))
                        .with_context(|| context!());
                    }
                }
            }
            _ => {
                return Err(anyhow!("not implemented yet: opcode {:#x}", opcode))
                    .with_context(|| context!());
            }
        }
        Ok(())
    }

    /// Load a value from a dram.
    fn load(&self, addr: u32, size: u32) -> Result<u32> {
        self.dram.load(addr, size)
    }

    /// Store a value to a dram.
    fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<()> {
        self.dram.store(addr, size, value)
    }

    /// Store a value to a CSR.
    fn store_csr(&mut self, addr: u32, value: u32) -> Result<()> {
        match addr {
            MSTATUS | MEPC | MCAUSE => {
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
            MSTATUS | MEPC | MCAUSE => {
                let addr = addr as usize;
                Ok(self.csrs[addr])
            }
            _ => Err(anyhow!("not implemented yet: csr {:#x}", addr)).with_context(|| context!()),
        }
    }
}
