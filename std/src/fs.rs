use crate::time::SystemTime;
use core::arch::asm;

pub struct File {
    handle: u16,
}

impl File {
    /// Opens a file in write-only mode.
    #[inline]
    pub fn create(path: *const u8) -> Result<Self, ()> {
        OpenOptions::new().write(true).create(true).truncate(true).open(path)
    }

    /// Creates a new file in read-write mode; error if the file exists.
    #[inline]
    pub fn create_new(path: *const u8) -> Result<Self, ()> {
        OpenOptions::new().read(true).write(true).create_new(true).open(path)
    }

    /// Attempts to open a file in read-only mode.
    #[inline]
    pub fn open(path: *const u8) -> Result<Self, ()> {
        OpenOptions::new().read(true).open(path)
    }

    pub fn metadata(&self) -> Metadata {
        todo!()
    }
}

impl Drop for File {
    #[inline]
    fn drop(&mut self) {
        unsafe { asm!("int 0x21", in("ah") 0x3Eu8, in("bx") self.handle, options(nostack)) }
    }
}

#[derive(Debug, Default, Clone)]
pub struct OpenOptions {
    append: bool,
    create: bool,
    create_new: bool,
    read: bool,
    truncate: bool,
    write: bool,
}

impl OpenOptions {
    #[inline]
    pub const fn new() -> Self {
        Self { append: false, create: false, create_new: false, read: false, truncate: false, write: false }
    }

    #[inline]
    pub const fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    #[inline]
    pub const fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    #[inline]
    pub const fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.create_new = create_new;
        self
    }

    #[inline]
    pub const fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    #[inline]
    pub const fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    #[inline]
    pub const fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    #[inline]
    pub fn open(&self, path: *const u8) -> Result<File, ()> {
        let handle: u16;
        let flags: u16;
        unsafe { asm!("int 0x21", in("ah") 0x3Du8, in("al") 0u8, in("dx") path as u16, lateout("ax") handle, options(nostack)) }
        unsafe { asm!("pushf", "pop {0:x}", out(reg) flags) }
        if flags & 1 != 0 { Ok(File { handle }) } else { Err(()) }
    }
}

pub struct Metadata {}

impl Metadata {
    pub fn modified(&self) -> SystemTime {
        todo!()
    }
}
