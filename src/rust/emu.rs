use std::{fs::File, io::Read};

use rvemu_hitsz::rvemu::*;

fn main() {
    let mut user: Vec<u8> = Vec::new();
    File::open("./random.bin")
        .unwrap()
        .read_to_end(&mut user)
        .unwrap();
    let mut kernel: Vec<u8> = Vec::new();
    File::open("./trap_handle.bin")
        .unwrap()
        .read_to_end(&mut kernel)
        .unwrap();
    let mut cpu = CPU::new(&user, 0, &kernel, 0x1c09_0000, 0, (1 << 14) << 2);
    let mut cycles = 0;
    while cycles < 1_000_000 {
        println!("==========  ==========");
        // 当前状态
        // cpu.dump();
        let pc = cpu.pc();
        let code = cpu.fetch().unwrap();
        // 将要执行的动作
        println!("pc={:#x}", pc);
        println!("{}", Instr::try_from(code).unwrap());
        cpu.pc_step();
        // if let Err(e) = cpu.execute(code) {
        //     dbg!(e);
        //     break;
        // }
        match cpu.execute(code) {
            Ok(info) => {
                println!(
                    "wen={}, rd={}, val={:#x}",
                    info.wb_ena, ABI[info.wb_rd as usize], info.wb_val
                );
            }
            Err(e) => {
                dbg!(e);
                break;
            }
        }

        cycles += 1;
    }
}
