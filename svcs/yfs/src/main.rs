use io_uring::squeue::Entry;
use io_uring::{opcode, types, IoUring};
use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::{fs, io};

fn main() -> io::Result<()> {
    let fd = fs::File::open("/dev/nvme0")?;
    let mut buf = vec![0; 1024];

    let nvme_read = opcode::UringCmd80::new(fd.as_raw_fd(), 1)
        .build()
        .user_data(0x42);

    println!("nvme_read: {:?}", nvme_read);

    Ok(())
}
