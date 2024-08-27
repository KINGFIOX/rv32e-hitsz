#ifndef __EMU_H__
#define __EMU_H__

#include <cstdint>

typedef struct WBStatus
{
    uint32_t wb_have_inst;
    uint32_t wb_pc;
    uint32_t wb_rd;
    uint32_t wb_val;
    uint32_t wb_ena;
    uint32_t inst_valid;
} WBStatus;

extern "C"
{
    extern uint64_t rvemu_new(
        uint8_t user_ptr[],
        uint32_t user_base,
        uint32_t user_len,
        uint8_t kernel_ptr[],
        uint32_t kernel_base,
        uint32_t kernel_len,
        uint32_t dram_base,
        uint32_t dram_size);

    extern void rvemu_free(uint64_t emu);
    extern WBStatus rvemu_execute(uint64_t emu, uint32_t inst);
    extern uint32_t rvemu_fetch(uint64_t emu);
    extern void rvemu_pc_step(uint64_t emu);
    extern void rvemu_dump(uint64_t emu);
    extern uint32_t rvemu_pc(uint64_t emu);
    extern void disasm(uint32_t inst);
}

#endif