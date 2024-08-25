use super::*;

pub type Reg = u32;

#[allow(clippy::upper_case_acronyms)]
pub type CSR = u32;

#[allow(clippy::upper_case_acronyms)]
pub enum Instr {
    // u
    LUI(Reg /* rd */, u32),
    AUIPC(Reg, u32),
    // j
    JAL(Reg /* rd */, i32),
    // b
    BEQ(Reg /* rs1 */, Reg /* rs2 */, i32),
    BNE(Reg, Reg, i32),
    BLT(Reg, Reg, i32),
    BGE(Reg, Reg, i32),
    BLTU(Reg, Reg, i32),
    BGEU(Reg, Reg, i32),
    // s
    SB(
        Reg, /* rd */
        i32, /* offset */
        Reg, /* base */
    ),
    SH(Reg, i32, Reg),
    SW(Reg, i32, Reg),
    // i
    ADDI(Reg /* rd */, Reg /* rs1 */, i32 /* imm */),
    ANDI(Reg, Reg, i32),
    ORI(Reg, Reg, i32),
    XORI(Reg, Reg, i32),
    SLLI(Reg, Reg, i32),
    SRLI(Reg, Reg, i32),
    SRAI(Reg, Reg, i32),
    SLTI(Reg, Reg, i32),
    SLTIU(Reg, Reg, i32),
    LB(
        Reg,
        /* rd */ i32,
        /* offset */ Reg, /* base */
    ),
    LH(Reg, i32, Reg),
    LW(Reg, i32, Reg),
    LBU(Reg, i32, Reg),
    LHU(Reg, i32, Reg),
    JALR(Reg, i32, Reg),
    // r
    ADD(Reg /* rd */, Reg /* rs1 */, Reg /* rs2 */),
    SUB(Reg, Reg, Reg),
    SLL(Reg, Reg, Reg),
    SLT(Reg, Reg, Reg),
    SLTU(Reg, Reg, Reg),
    XOR(Reg, Reg, Reg),
    SRL(Reg, Reg, Reg),
    SRA(Reg, Reg, Reg),
    OR(Reg, Reg, Reg),
    AND(Reg, Reg, Reg),
    // zicsr
    ECALL,
    ERET,
    CSRRW(Reg /* rd */, CSR, Reg /* rs1 */),
    CSRRS(Reg, CSR, Reg),
    CSRRC(Reg, CSR, Reg),
    CSRRWI(Reg /* rd */, CSR, u32 /* zimm */),
    CSRRSI(Reg, CSR, u32),
    CSRRCI(Reg, CSR, u32),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // u
            Instr::LUI(rd, umm) => write!(f, "lui {}, {:#x}", ABI[*rd as usize], *umm >> 12),
            Instr::AUIPC(rd, umm) => write!(f, "auipc {}, {:#x}", ABI[*rd as usize], *umm),
            // j
            Instr::JAL(rd, imm) => write!(f, "jal {}, {:#x}", ABI[*rd as usize], imm),
            // b
            Instr::BEQ(rs1, rs2, imm) => write!(
                f,
                "beq {}, {}, {:#x}",
                ABI[*rs1 as usize], ABI[*rs2 as usize], imm
            ),
            Instr::BNE(rs1, rs2, imm) => write!(
                f,
                "bne {}, {}, {:#x}",
                ABI[*rs1 as usize], ABI[*rs2 as usize], imm
            ),
            Instr::BLT(rs1, rs2, imm) => write!(
                f,
                "blt {}, {}, {:#x}",
                ABI[*rs1 as usize], ABI[*rs2 as usize], imm
            ),
            Instr::BGE(rs1, rs2, imm) => write!(
                f,
                "bge {}, {}, {:#x}",
                ABI[*rs1 as usize], ABI[*rs2 as usize], imm
            ),
            Instr::BLTU(rs1, rs2, imm) => write!(
                f,
                "bltu {}, {}, {:#x}",
                ABI[*rs1 as usize], ABI[*rs2 as usize], imm
            ),
            Instr::BGEU(rs1, rs2, imm) => write!(
                f,
                "bgeu {}, {}, {:#x}",
                ABI[*rs1 as usize], ABI[*rs2 as usize], imm
            ),
            // s
            Instr::SB(rd, offset, base) => write!(
                f,
                "sb {}, {:#x}({})",
                ABI[*rd as usize], offset, ABI[*base as usize]
            ),
            Instr::SH(rd, offset, base) => write!(
                f,
                "sh {}, {:#x}({})",
                ABI[*rd as usize], offset, ABI[*base as usize]
            ),
            Instr::SW(rd, offset, base) => write!(
                f,
                "sw {}, {:#x}({})",
                ABI[*rd as usize], offset, ABI[*base as usize]
            ),
            // i
            Instr::ADDI(rd, rs1, imm) => write!(
                f,
                "addi {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::ANDI(rd, rs1, imm) => write!(
                f,
                "andi {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::ORI(rd, rs1, imm) => write!(
                f,
                "ori {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::XORI(rd, rs1, imm) => write!(
                f,
                "xori {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::SLLI(rd, rs1, imm) => write!(
                f,
                "slli {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::SRLI(rd, rs1, imm) => write!(
                f,
                "srli {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::SRAI(rd, rs1, imm) => write!(
                f,
                "srai {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::SLTI(rd, rs1, imm) => write!(
                f,
                "slti {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::SLTIU(rd, rs1, imm) => write!(
                f,
                "sltiu {}, {}, {:#x}",
                ABI[*rd as usize], ABI[*rs1 as usize], imm
            ),
            Instr::LB(rd, offset, base) => write!(
                f,
                "lb {}, {:#x}({})",
                ABI[*rd as usize], offset, ABI[*base as usize]
            ),
            Instr::LH(rd, offset, base) => write!(
                f,
                "lh {}, {:#x}({})",
                ABI[*rd as usize], offset, ABI[*base as usize]
            ),
            Instr::LW(rd, offset, base) => write!(
                f,
                "lw {}, {:#x}({})",
                ABI[*rd as usize], offset, ABI[*base as usize]
            ),
            Instr::LBU(rd, offset, base) => write!(
                f,
                "lbu {}, {:#x}({})",
                ABI[*rd as usize], offset, ABI[*base as usize]
            ),
            Instr::LHU(rd, offset, base) => write!(
                f,
                "lhu {}, {:#x}({})",
                ABI[*rd as usize], offset, ABI[*base as usize]
            ),
            Instr::JALR(rd, imm, base) => write!(
                f,
                "jalr {}, {:#x}({})",
                ABI[*rd as usize], imm, ABI[*base as usize]
            ),
            // r
            Instr::ADD(rd, rs1, rs2) => write!(
                f,
                "add {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::SUB(rd, rs1, rs2) => write!(
                f,
                "sub {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::AND(rd, rs1, rs2) => write!(
                f,
                "and {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::XOR(rd, rs1, rs2) => write!(
                f,
                "xor {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::SLL(rd, rs1, rs2) => write!(
                f,
                "sll {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::SRL(rd, rs1, rs2) => write!(
                f,
                "srl {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::SRA(rd, rs1, rs2) => write!(
                f,
                "sra {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::SLT(rd, rs1, rs2) => write!(
                f,
                "slt {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::OR(rd, rs1, rs2) => write!(
                f,
                "or {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            Instr::SLTU(rd, rs1, rs2) => write!(
                f,
                "sltu {}, {}, {}",
                ABI[*rd as usize], ABI[*rs1 as usize], ABI[*rs2 as usize]
            ),
            // zicsr
            Instr::ECALL => write!(f, "ecall"),
            Instr::ERET => write!(f, "eret"),
            Instr::CSRRW(rd, csr, rs1) => write!(
                f,
                "csrrw {}, {}, {}",
                ABI[*rd as usize],
                csr_abi(csr),
                ABI[*rs1 as usize]
            ),
            Instr::CSRRS(rd, csr, rs1) => write!(
                f,
                "csrrs {}, {}, {}",
                ABI[*rd as usize],
                csr_abi(csr),
                ABI[*rs1 as usize]
            ),
            Instr::CSRRC(rd, csr, rs1) => write!(
                f,
                "csrrc {}, {}, {}",
                ABI[*rd as usize],
                csr_abi(csr),
                ABI[*rs1 as usize]
            ),
            Instr::CSRRWI(rd, csr, zimm) => write!(
                f,
                "csrrwi {}, {}, {:#x}",
                ABI[*rd as usize],
                csr_abi(csr),
                zimm
            ),
            Instr::CSRRSI(rd, csr, zimm) => write!(
                f,
                "csrrsi {}, {}, {:#x}",
                ABI[*rd as usize],
                csr_abi(csr),
                zimm
            ),
            Instr::CSRRCI(rd, csr, zimm) => write!(
                f,
                "csrrci {}, {}, {:#x}",
                ABI[*rd as usize],
                csr_abi(csr),
                zimm
            ),
        }
    }
}

impl TryFrom<u32> for Instr {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        let opcode = value & 0x0000007f;
        let rd = (value & 0x00000f80) >> 7;
        let rs1 = (value & 0x000f8000) >> 15;
        let rs2 = (value & 0x01f00000) >> 20;
        let funct3 = (value & 0x00007000) >> 12;
        let funct7 = (value & 0xfe000000) >> 25;
        match opcode {
            // i
            0x13 => {
                // imm[11:0] = inst[31:20]
                let imm = (value & 0xfff00000) as i32 >> 20;
                // "The shift amount is encoded in the lower 6 bits of the I-immediate field for RV64I."
                match funct3 {
                    0x0 => Ok(Self::ADDI(rd, rs1, imm)),
                    0x1 => Ok(Self::SLLI(rd, rs1, imm)),
                    0x2 => Ok(Self::SLTI(rd, rs1, imm)),
                    0x3 => Ok(Self::SLTIU(rd, rs1, imm)),
                    0x4 => Ok(Self::XORI(rd, rs1, imm)),
                    0x5 => {
                        let shamt = imm & 0x3f;
                        match funct7 >> 1 {
                            // srli
                            0x00 => Ok(Self::SRLI(rd, rs1, shamt)),
                            // srai
                            0x10 => Ok(Self::SRAI(rd, rs1, shamt)),
                            _ => Err(anyhow!(
                                "not implemented yet: opcode {:#x} funct3 {:#x} funct7 {:#x}",
                                opcode,
                                funct3,
                                funct7
                            ))
                            .with_context(|| context!()),
                        }
                    }
                    0x6 => Ok(Self::ORI(rd, rs1, imm)),
                    0x7 => Ok(Self::ANDI(rd, rs1, imm)),
                    _ => Err(anyhow!(
                        "not implemented yet: opcode {:#x} funct3 {:#x}",
                        opcode,
                        funct3
                    ))
                    .with_context(|| context!()),
                }
            }
            0x03 => {
                // imm[11:0] = inst[31:20]
                let offset = (value as i32) >> 20;
                match funct3 {
                    0x0 => Ok(Self::LB(rd, offset, rs1)),
                    0x1 => Ok(Self::LH(rd, offset, rs1)),
                    0x2 => Ok(Self::LW(rd, offset, rs1)),
                    0x4 => Ok(Self::LBU(rd, offset, rs1)),
                    0x5 => Ok(Self::LHU(rd, offset, rs1)),
                    _ => Err(anyhow!(
                        "not implemented yet: opcode {:#x} funct3 {:#x}",
                        opcode,
                        funct3
                    ))
                    .with_context(|| context!()),
                }
            }
            0x67 => {
                let imm = ((value & 0xfff00000) as i32) >> 20;
                Ok(Self::JALR(rd, imm, rs1))
            }
            // u
            0x17 => {
                let imm = value & 0xfffff000;
                Ok(Self::AUIPC(rd, imm))
            }
            0x37 => {
                let imm = value & 0xfffff000;
                Ok(Self::LUI(rd, imm))
            }
            // j
            0x6f => {
                // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
                let imm = (((value & 0x8000_0000) as i32  >> 11) as u32) // imm[20]
                    | (value & 0xff000) // imm[19:12]
                    | ((value >> 9) & 0x800) // imm[11]
                    | ((value >> 20) & 0x7fe); // imm[10:1]
                Ok(Self::JAL(rd, imm as i32))
            }
            // b
            0x63 => {
                // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
                let imm = (((value & 0x80000000) as i32  >> 19) as u32)
                    | ((value & 0x80) << 4) // imm[11]
                    | ((value >> 20) & 0x7e0) // imm[10:5]
                    | ((value >> 7) & 0x1e); // imm[4:1]
                let imm = imm as i32;

                match funct3 {
                    0x0 => Ok(Self::BEQ(rs1, rs2, imm)),
                    0x1 => Ok(Self::BNE(rs1, rs2, imm)),
                    0x4 => Ok(Self::BLT(rs1, rs2, imm)),
                    0x5 => Ok(Self::BGE(rs1, rs2, imm)),
                    0x6 => Ok(Self::BLTU(rs1, rs2, imm)),
                    0x7 => Ok(Self::BGEU(rs1, rs2, imm)),
                    _ => Err(anyhow!(
                        "not implemented yet: opcode {:#x} funct3 {:#x}",
                        opcode,
                        funct3
                    ))
                    .with_context(|| context!()),
                }
            }
            // s
            0x23 => {
                // imm[11:5|4:0] = inst[31:25|11:7]
                let offset = (((value & 0xfe000000) as i32 >> 20) as u32) | ((value >> 7) & 0x1f);
                let offset = offset as i32;
                match funct3 {
                    0x0 => Ok(Self::SB(rs2, offset, rs1)), // sb
                    0x1 => Ok(Self::SH(rs2, offset, rs1)), // sh
                    0x2 => Ok(Self::SW(rs2, offset, rs1)), // sw
                    _ => Err(anyhow!(
                        "not implemented yet: opcode {:#x} funct3 {:#x}",
                        opcode,
                        funct3
                    ))
                    .with_context(|| context!()),
                }
            }
            // r
            0x33 => match (funct3, funct7) {
                (0x0, 0x00) => Ok(Self::ADD(rd, rs1, rs2)),
                (0x0, 0x20) => Ok(Self::SUB(rd, rs1, rs2)),
                (0x1, 0x00) => Ok(Self::SLL(rd, rs1, rs2)),
                (0x2, 0x00) => Ok(Self::SLT(rd, rs1, rs2)),
                (0x3, 0x00) => Ok(Self::SLTU(rd, rs1, rs2)),
                (0x4, 0x00) => Ok(Self::XOR(rd, rs1, rs2)),
                (0x5, 0x00) => Ok(Self::SRL(rd, rs1, rs2)),
                (0x5, 0x20) => Ok(Self::SRA(rd, rs1, rs2)),
                (0x6, 0x00) => Ok(Self::OR(rd, rs1, rs2)),
                (0x7, 0x00) => Ok(Self::AND(rd, rs1, rs2)),
                _ => Err(anyhow!(
                    "not implemented yet: opcode {:#x} funct3 {:#x} funct7 {:#x}",
                    opcode,
                    funct3,
                    funct7
                ))
                .with_context(|| context!()),
            },
            0x73 => {
                let csr_addr = (value & 0xfff00000) >> 20;
                let zimm = rs1;
                match funct3 {
                    0x0 => match rs2 {
                        0x0 => Ok(Self::ECALL),
                        0x02 => Ok(Self::ERET), // uret
                        _ => {
                            Err(anyhow!(
                                "not implemented yet: opcode {:#x} funct3 {:#x} rs2 {:#x}",
                                opcode,
                                funct3,
                                rs2
                            ))
                            .with_context(|| context!())
                        }
                    },
                    0x08 /* sret */ | 0x18 /* mret */ => Ok(Self::ERET),
                    0x1 => Ok(Self::CSRRW(rd, csr_addr, rs1)),
                    0x2 => Ok(Self::CSRRS(rd, csr_addr, rs1)),
                    0x3 => Ok(Self::CSRRC(rd, csr_addr, rs1)),
                    0x5 => Ok(Self::CSRRWI(rd, csr_addr, zimm)),
                    0x6 => Ok(Self::CSRRSI(rd, csr_addr, zimm)),
                    0x7 => Ok(Self::CSRRCI(rd, csr_addr, zimm)),
                    _ => {
                        Err(anyhow!(
                            "not implemented yet: opcode {:#x} funct3 {:#x}",
                            opcode,
                            funct3
                        ))
                        .with_context(|| context!())
                    }
                }
            }
            _ => Err(anyhow!("not implemented yet: opcode {:#x}", opcode))
                .with_context(|| context!()),
        }
    }
}
