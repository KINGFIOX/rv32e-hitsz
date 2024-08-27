#include <iostream>
#include <fstream>
#include <vector>

#include "emu.h"

int main(void)
{
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
    uint64_t cpu = rvemu_new(user_ptr, 0, user_len, kernel_ptr, 0x1c09'0000, kernel_len, 0x0000'0000, 0xffff'ff00);

    while (true)
    {
        std::cout << "========== " << rvemu_pc(cpu) << " ==========" << std::endl;
        rvemu_dump(cpu);
        uint32_t code = rvemu_fetch(cpu);
        disasm(code);
        rvemu_pc_step(cpu);
        WBStatus info = rvemu_execute(cpu, code);
        if (!info.inst_valid)
        {
            break;
        }
    }

    return (int)cpu;
}