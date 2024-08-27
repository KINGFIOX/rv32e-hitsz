#ifndef __DUT_H__
#define __DUT_H__

#include "VminiRV_SoC.h"
#include "emu.h"

#include <sys/types.h>
#include <verilated.h>
#include <verilated_vcd_c.h>
#include <string>
#include <iostream>

class DUT
{
public:
    VminiRV_SoC *dut = new VminiRV_SoC;
    VerilatedVcdC *vrltr_dump = new VerilatedVcdC; // dump 文件句柄
    size_t cnt = 0;

    DUT(std::string &path)
    {
        Verilated::traceEverOn(true);

        // trace dump
        this->dut->trace(vrltr_dump, 99);
        this->vrltr_dump->open(path.c_str());
    }

    ~DUT()
    {
        // close trace
        vrltr_dump->close();
        vrltr_dump = nullptr;
        delete vrltr_dump;
        delete dut;
    }

    /**
     * @brief 打一拍
     *
     * @return WBInfo
     */
    WBInfo tick(void)
    {
        cnt++;
        dut->clock = 0;
        dut->eval();
        vrltr_dump->dump((vluint64_t)(10 * cnt - 1));
        // Repeat for the positive edge of the clock
        dut->clock = 1;
        dut->eval();
        vrltr_dump->dump((vluint64_t)(10 * cnt));
        // Now the negative edge
        dut->clock = 0;
        dut->eval();
        vrltr_dump->dump((vluint64_t)(10 * cnt + 5));
        vrltr_dump->flush();

        // 返回值
        WBInfo ret;
        ret.wb_have_inst = dut->io_dbg_wb_have_inst;
        ret.wb_pc = dut->io_dbg_wb_pc;
        ret.wb_ena = dut->io_dbg_wb_ena;
        ret.wb_rd = dut->io_dbg_wb_reg;
        ret.wb_val = dut->io_dbg_wb_value;
        ret.inst_valid = dut->io_dbg_inst_valid;
        return ret;
    }

    void reset()
    {
        std::cout << "[my-cpu] Resetting ..." << std::endl;
        dut->reset = 1;
        for (int i = 0; i < 20; i++)
        {
            tick();
        }
        dut->reset = 0;
        std::cout << "[my-cpu] Reset done." << std::endl;
    }

    void wb_dump()
    {
        printf("PC=0x%8.8x, WBEn = %d, WReg = %d, WBValue = 0x%8.8x\n", dut->io_dbg_wb_pc, dut->io_dbg_wb_ena, dut->io_dbg_wb_reg, dut->io_dbg_wb_value);
    }
};
#endif