use super::*;
use anyhow::{anyhow, Context, Result};
use std::fmt::Display;

mod cpu;
mod dram;
mod instr;
mod irom;

use dram::*;
use irom::*;

pub use cpu::*;
pub use instr::*;

const MSTATUS: u32 = 0x0300;
const MEPC: u32 = 0x0341;
const MCAUSE: u32 = 0x0342;

const ABI: [&str; 32] = [
    "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ", " a1 ",
    " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ", " s6 ", " s7 ",
    " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
];

fn csr_abi(csr: &CSR) -> String {
    match *csr {
        MCAUSE => "mcause".to_string(),
        MSTATUS => "mstatus".to_string(),
        MEPC => "mepc".to_string(),
        _ => format!("csr_{:#x}", csr),
    }
}
