#![allow(unused, dead_code)]
#![no_std]
#![no_main]

mod linux;
mod rt;
mod utils;

use linux::{CloneFlags, WaitFlags};
use utils::Args;

fn main(args: Args) -> isize {
    for (i, arg) in args.iter().enumerate() {
        println!("arg[{}]: {:?}", i, arg);
    }

    let flags = CloneFlags::NEWNS
        | CloneFlags::NEWPID
        | CloneFlags::NEWUTS
        | CloneFlags::NEWNET
        | CloneFlags::NEWIPC
        | CloneFlags::SIGCHLD;

    match linux::clone(flags, None) {
        Ok(0) => 0xff,
        Ok(pid) => {
            let mut status = 0;
            let _ = linux::wait4(pid as isize, Some(&mut status), WaitFlags::EMPTY);
            println!("wait4 status: {:08x}", status);
            0
        }
        Err(err) => {
            eprintln!("clone failed: {:?}", err);
            1
        }
    }
}
