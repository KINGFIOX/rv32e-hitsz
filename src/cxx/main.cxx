#include <iostream>
#include <fstream>
#include <vector>
#include <format>

#include "emu.h"
#include "dut.h"

#include <verilated.h>
#include <verilated_vcd_c.h>

int main(int argc, char const *argv[])
{
    std::string trace;
    if (argc == 2)
    {
        trace = std::format("waveform/{}.vcd", argv[1]);
    }
    else
    {
        trace = std::format("waveform/start.vcd", argv[1]);
    }
    auto top = new DUT(trace);
    top->reset();
    top->tick();
    top->tick();

    // user
    std::ifstream user_file("./start.bin", std::ios::binary);
    std::vector<uint8_t> user_code((std::istreambuf_iterator<char>(user_file)), std::istreambuf_iterator<char>());
    auto user_ptr = user_code.data();
    auto user_len = (uint32_t)user_code.size();
    // kernel
    std::ifstream kernel_file("./trap_handle.bin", std::ios::binary);
    std::vector<uint8_t> kernel_code((std::istreambuf_iterator<char>(kernel_file)), std::istreambuf_iterator<char>());
    auto kernel_ptr = kernel_code.data();
    auto kernel_len = (uint32_t)kernel_code.size();
    uint64_t emu = rvemu_new(user_ptr, 0, user_len, kernel_ptr, 0x1c09'0000, kernel_len, 0x0000'0000, 0xffff'f000);

    while (true)
    {
        std::cout << "========== " << rvemu_pc(emu) << " ==========" << std::endl;
        // rvemu_dump(emu);
        uint32_t code = rvemu_fetch(emu);
        disasm(code);
        WBInfo info_dut = top->tick();
        if (!info_dut.inst_valid)
        {
            continue;
        }
        rvemu_pc_step(emu);
        WBInfo info_emu = rvemu_execute(emu, code);
        top->wb_dump();
        printf("PC=0x%8.8x, WBEn = %d, WReg = %d, WBValue = 0x%8.8x\n", info_emu.wb_pc, info_emu.wb_ena, info_emu.wb_rd, info_emu.wb_val);
        if (!info_emu.inst_valid)
        {
            break;
        }
    }

    return (int)emu;
}
