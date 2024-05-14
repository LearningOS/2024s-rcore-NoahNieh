//! Process management syscalls

use core::mem::size_of;

use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{translated_byte_buffer, VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task_info,mmap, munmap, suspend_current_and_run_next, TaskStatus
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let token = current_user_token();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    // 从内存中取出，按页表分为多个buffer
    let buffers= translated_byte_buffer(token, _ts as *const u8, size_of::<TimeVal>());
    // 转成指针
    let mut time_val_p = &time_val as *const _ as *const u8;
    // copy到对应的内存中
    for ele in buffers {
        unsafe {
            time_val_p.copy_to(ele.as_mut_ptr(), ele.len());
            time_val_p = time_val_p.add(ele.len());
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let info = get_current_task_info();
    let task_info = TaskInfo {
        status : info.0,
        syscall_times : info.1,
        time : get_time_ms() - info.2,
    };
    let buffer = translated_byte_buffer(current_user_token(), _ti as *const u8, size_of::<TaskInfo>());
    let mut task_info_p = &task_info as *const _ as *const u8;
    for ele in buffer {
        unsafe {
            task_info_p.copy_to(ele.as_mut_ptr(), ele.len());
            task_info_p = task_info_p.add(ele.len());
        }
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap ");
    let va = VirtAddr::from(_start);
    if !va.aligned() || (_port & !0x7) != 0 || (_port & 0x7) == 0 {
        return -1;
    }
    mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap ");
    munmap(_start, _len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
