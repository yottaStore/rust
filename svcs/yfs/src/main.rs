extern crate core;

use io_uring::squeue::Entry;
use io_uring::types::Fd;
use io_uring::{opcode, types, IoUring};

use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::{fs, io};

pub type __u8 = ::std::os::raw::c_uchar;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __u32 = ::std::os::raw::c_uint;
pub type __u64 = ::std::os::raw::c_ulonglong;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nvme_uring_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub rsvd1: __u16,
    pub nsid: __u32,
    pub cdw2: __u32,
    pub cdw3: __u32,
    pub metadata: __u64,
    pub addr: __u64,
    pub metadata_len: __u32,
    pub data_len: __u32,
    pub cdw10: __u32,
    pub cdw11: __u32,
    pub cdw12: __u32,
    pub cdw13: __u32,
    pub cdw14: __u32,
    pub cdw15: __u32,
    pub timeout_ms: __u32,
    pub rsvd2: __u32,
}

fn main() -> io::Result<()> {
    let mut builder = IoUring::builder();
    let mut ring = builder.build(128)?;

    let fd = fs::File::open("/dev/nvme0")?;

    let mut buff = [0u8; 4096];
    let mut buf: *mut u8 = &mut buff[0];
    let tfd = Fd(fd.as_raw_fd());

    let lba: u64 = 1000;
    let num_blocks: u32 = 1;
    // TODO: check correct cmd_opcode
    let cmd_op: u32 = 0x80;
    let opcode: u8 = 0x2;
    let data_addr = buf as u64;
    let data_len = 1 as u32;
    let cdw10 = (lba & 0xffffffff) as u32;
    let cdw11 = (lba >> 32) as u32;
    let cdw12 = num_blocks - 1;

    let cmd = nvme_uring_cmd {
        opcode,
        // TODO: find nsid
        nsid: 1,
        addr: data_addr,
        data_len,
        cdw10,
        cdw11,
        cdw12,
        ..Default::default()
    };

    let mut cmd_bytes = [0u8; 80];
    unsafe {
        cmd_bytes
            .as_mut_ptr()
            .cast::<nvme_uring_cmd>()
            .write_unaligned(cmd);
    }

    let nvme_read = opcode::UringCmd80::new(tfd, cmd_op)
        .cmd(cmd_bytes)
        .build()
        .user_data(0x22);

    println!("nvme_read: {:?}", nvme_read);

    unsafe {
        ring.submission()
            .push(&nvme_read)
            .expect("submission queue is full");
    }

    ring.submit_and_wait(1)?;

    let cqe = ring.completion().next().expect("completion queue is empty");

    assert_eq!(cqe.user_data(), 0x42);
    assert!(cqe.result() >= 0, "read error: {}", cqe.result());

    let content = std::str::from_utf8(&buff).unwrap();
    println!("bytes read: {:?}", content);

    Ok(())
}
