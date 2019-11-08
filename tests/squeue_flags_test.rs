use linux_io_uring::{opcode, squeue, IoUring};

#[test]
fn test_io_drain() -> anyhow::Result<()> {
    let mut io_uring = IoUring::new(4)?;

    unsafe {
        let mut queue = io_uring.submission().available();
        queue
            .push(opcode::Nop::new().build().user_data(0x01))
            .map_err(drop)
            .expect("queue is full");
        queue
            .push(opcode::Nop::new().build().user_data(0x02))
            .map_err(drop)
            .expect("queue is full");
        queue
            .push(
                opcode::Nop::new()
                    .build()
                    .user_data(0x03)
                    .flags(squeue::Flags::IO_DRAIN),
            )
            .map_err(drop)
            .expect("queue is full");
        queue
            .push(opcode::Nop::new().build().user_data(0x04))
            .map_err(drop)
            .expect("queue is full");
    }

    io_uring.submit_and_wait(4)?;

    let cqes = io_uring
        .completion()
        .available()
        .map(|entry| entry.user_data())
        .collect::<Vec<_>>();

    assert!(cqes[0] == 0x01 || cqes[0] == 0x02);
    assert!(cqes[1] == 0x01 || cqes[1] == 0x02);
    assert_ne!(cqes[0], cqes[1]);
    assert_eq!(cqes[2], 0x03);
    assert_eq!(cqes[3], 0x04);

    Ok(())
}

#[test]
fn test_io_link() -> anyhow::Result<()> {
    let mut io_uring = IoUring::new(4)?;

    unsafe {
        let mut queue = io_uring.submission().available();
        queue
            .push(
                opcode::Nop::new()
                    .build()
                    .user_data(0x01)
                    .flags(squeue::Flags::IO_LINK),
            )
            .map_err(drop)
            .expect("queue is full");
        queue
            .push(
                opcode::Nop::new()
                    .build()
                    .user_data(0x02)
                    .flags(squeue::Flags::IO_LINK),
            )
            .map_err(drop)
            .expect("queue is full");
        queue
            .push(
                opcode::Nop::new()
                    .build()
                    .user_data(0x03)
                    .flags(squeue::Flags::IO_LINK),
            )
            .map_err(drop)
            .expect("queue is full");
        queue
            .push(opcode::Nop::new().build().user_data(0x04))
            .map_err(drop)
            .expect("queue is full");
    }

    io_uring.submit_and_wait(4)?;

    let cqes = io_uring
        .completion()
        .available()
        .map(|entry| entry.user_data())
        .collect::<Vec<_>>();

    assert_eq!(cqes, [0x01, 0x02, 0x03, 0x04]);

    Ok(())
}
