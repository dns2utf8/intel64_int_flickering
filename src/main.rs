use std::thread::{spawn, park};
use std::ptr::{write_volatile, read_volatile};

type TestType = isize;

static mut SHARED: TestType  = 10;

fn main() {

    for _ in 0..64 {
        spawn(|| {
            unsafe {
                let p = &mut SHARED as *mut TestType;
                loop {
                    write_volatile(p, -1);
                    write_volatile(p, 0);
                }
            };
        });
    }

    for _ in 0..64 {
        spawn(|| {
            unsafe {
                let p = &mut SHARED as *mut TestType;
                let mut tmp: TestType;
                loop {
                    tmp = read_volatile(p);
                    if tmp != 0 && tmp != -1 {
                        println!("found: {}", tmp);
                    }
                }
            };
        });
    }

    // stop the main thread so the workers survive
    park();
    println!("unparked main for no reason ...");
}
