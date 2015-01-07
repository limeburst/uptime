extern crate libc;
extern crate time;
extern crate utmp;

use libc::funcs::bsd44::sysctl;
use libc::types::os::common::posix01::timeval;
use std::mem::size_of_val;
use std::ptr::null_mut;

static CTL_VM: libc::c_int = 2;
static CTL_KERN: libc::c_int = 1;
static KERN_BOOTTIME: libc::c_int = 21;
static VM_LOADAVG: libc::c_int = 2;

struct LoadAverage {
    ldavg: [u32; 3],
    fscale: uint,
}

fn getusercount() -> int {
    let utmpx = utmp::getutmpx();
    let mut usercount = 0;
    for record in utmpx.iter() {
        match record.ut_type {
            utmp::UtmpxRecordType::UserProcess => usercount += 1,
            _ => continue
        }
    }
    usercount
}

fn getloadavg() -> LoadAverage {
    let mut mib = [CTL_VM, VM_LOADAVG];
    let mut loadavg = LoadAverage {
        ldavg: [0, 0, 0],
        fscale: 0
    };
    let mut size: libc::size_t = size_of_val(&loadavg) as libc::size_t;
    unsafe {
        sysctl(&mut mib[0], 2,
               &mut loadavg as *mut LoadAverage as *mut libc::c_void,
               &mut size, null_mut(), 0);
    }
    loadavg
}

fn getboottime() -> timeval {
    let mut mib = [CTL_KERN, KERN_BOOTTIME];
    let mut boottime = timeval {
        tv_sec: 0,
        tv_usec: 0
    };
    let mut size: libc::size_t = size_of_val(&boottime) as libc::size_t;
    unsafe {
        sysctl(&mut mib[0], 2,
               &mut boottime as *mut timeval as *mut libc::c_void,
               &mut size, null_mut(), 0);
    }
    boottime
}

fn main() {
    let now = time::now();
    let loadavg = getloadavg();
    let usercount = getusercount();
    let seconds = now.to_timespec().sec - getboottime().tv_sec;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    print!("{}  up ", time::strftime("%H:%M", &now).unwrap());
    if days > 0 {
        print!("{} ", days);
        if days == 1 {
            print!("day, ");
        } else {
            print!("days, ");
        }
    };
    if hours > 0 && minutes > 0 {
        print!("{:2}:{:02}, ", hours % 24, minutes % 60);
    } else if hours > 0 {
        print!("{} ", hours % 24);
        if hours == 1 {
            print!("hr, ");
        } else {
            print!("hrs, ");
        }
    } else if minutes > 0 {
        print!("{} ", minutes % 60);
        if minutes == 1 {
            print!("min, ");
        } else {
            print!("mins, ");
        }
    } else {
        print!("{} ", seconds % 60);
        if seconds == 1 {
            print!("sec, ");
        } else {
            print!("secs, ");
        }
    }
    print!("{} ", usercount);
    if usercount == 1 {
        print!("user, ");
    } else {
        print!("users, ");
    }
    print!("load averages:");
    for avg in loadavg.ldavg.iter() {
        print!(" {}", std::f32::to_str_digits(*avg as f32 / loadavg.fscale as f32, 2));
    }
    println!("");
}
