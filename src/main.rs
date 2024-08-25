use std::{fs::File, io::Read};

use rvemu_hitsz::rvemu::*;

fn main() {
    let mut user: Vec<u8> = Vec::new();
    File::open("./start.bin")
        .unwrap()
        .read_to_end(&mut user)
        .unwrap();
    let mut kernel: Vec<u8> = Vec::new();
    File::open("./trap_handle.bin")
        .unwrap()
        .read_to_end(&mut kernel)
        .unwrap();
    let mut cpu = CPU::new(&user, 0, &kernel, 0x1c09_0000, 0, 1 << 31);
    loop {
        println!("==========  ==========");
        // 当前状态
        cpu.dump();
        let code = cpu.fetch().unwrap();
        // 将要执行的动作
        println!("{}", Instr::try_from(code).unwrap());
        cpu.pc_step();
        if let Err(e) = cpu.execute(code) {
            dbg!(e);
            break;
        }
    }
}
