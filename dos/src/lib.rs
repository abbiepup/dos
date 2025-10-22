#![no_std]

use core::marker::PhantomData;

pub mod fs;
pub mod time;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Psp {
    int20h: [u8; 2],
    last_block_seg: u16,
    terminate_ptr: FarPtr<()>,
    ctrl_break_ptr: FarPtr<()>,
    critical_error_ptr: FarPtr<()>,
    parent_psp: u16,
    file_handles: [u8; 20],
    env_segment: u16,
    dos_sp: [u8; 4],
    dispatcher: [u8; 2],
    reserved0: [u8; 10],
    fcb1: [u8; 16],
    fcb2: [u8; 20],
    _unused: [u8; 36],
    cmd_tail: [u8; 128],
}

impl Psp {
    pub const SIZE: usize = size_of::<Self>();

    #[inline]
    pub const fn parent(&self) -> Option<&'static Psp> {
        if self.parent_psp == 0 { None } else { Some(unsafe { &*(FarPtr::new(self.parent_psp, 0).as_ptr()) }) }
    }

    //     pub fn args(&self) -> (usize, &[*const c_char]) {
    //         let len = self.cmd_tail[0] as usize;
    //         let tail = &self.cmd_tail[1..=len.min(127)];

    //         let mut argc = 0;
    //         let mut argv: [*const c_char; 16] = [core::ptr::null(); 16];
    //         let mut index = 0;

    //         while index < tail.len() {
    //             while index < tail.len() && (tail[index] == b' ' || tail[index] == b'\t') {
    //                 index += 1;
    //             }

    //             if index >= tail.len() {
    //                 break;
    //             }

    //             let start = index;
    //             let mut in_quote = false;

    //             while index < tail.len() {
    //                 let b = tail[index];
    //                 if b == b'"' {
    //                     in_quote = !in_quote;
    //                     index += 1;
    //                 } else if !in_quote && (b == b' ' || b == b'\t') {
    //                     break;
    //                 } else {
    //                     index += 1;
    //                 }
    //             }

    //             argv[argc] = tail[start..index].as_ptr() as *const c_char;
    //             argc += 1;

    //             index += 1;
    //         }

    //         (argc, argv)
    //     }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FarPtr<T> {
    segment: u16,
    offset: u16,
    phantom: PhantomData<*mut T>,
}

impl<T> FarPtr<T> {
    #[inline]
    pub const fn new(segment: u16, offset: u16) -> Self {
        Self { segment, offset, phantom: PhantomData }
    }

    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        (((self.segment as u32) << 4) + (self.offset as u32)) as *mut T
    }
}
