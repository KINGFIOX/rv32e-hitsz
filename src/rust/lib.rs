pub mod rvemu;
pub mod utils;

use rvemu::*;

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rvemu_new(
    user_ptr: *const u8,
    user_base: u32,
    user_len: u32,
    kernel_ptr: *const u8,
    kernel_base: u32,
    kernel_len: u32,
    stack_base: u32,
    stack_size: u32,
) -> *mut CPU {
    let user = std::slice::from_raw_parts(user_ptr, user_len as usize);
    let kernel = std::slice::from_raw_parts(kernel_ptr, kernel_len as usize);
    let cpu = CPU::new(user, user_base, kernel, kernel_base, stack_base, stack_size);
    Box::into_raw(Box::new(cpu))
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rvemu_free(cpu: *mut CPU) {
    if cpu.is_null() {
        return;
    }
    let _ = Box::from_raw(cpu);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rvemu_execute(cpu: *mut CPU, inst: u32) -> WBStatus {
    let cpu = &mut *cpu;
    match cpu.execute(inst) {
        Ok(wb_status) => wb_status,
        Err(_) => WBStatus::default(),
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rvemu_fetch(cpu: *const CPU) -> u32 {
    let cpu = &*cpu;
    // 1.
    cpu.fetch().unwrap()
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rvemu_pc_step(cpu: *mut CPU) {
    let cpu = &mut *cpu;
    cpu.pc_step();
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rvemu_dump(cpu: *const CPU) {
    let cpu = &*cpu;
    cpu.dump();
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rvemu_pc(cpu: *const CPU) -> u32 {
    let cpu = &*cpu;
    cpu.pc()
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn disasm(inst: u32) {
    println!("{}", Instr::try_from(inst).unwrap());
}
