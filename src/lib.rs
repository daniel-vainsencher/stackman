extern crate time;

use time::precise_time_ns;

#[macro_export]
macro_rules! stack {
    ($name:expr, $work:expr) => {
        {
            let work = {|| $work};
            println!("Push {}", $name);
            let before = precise_time_ns();
            let value = work();
            let after = precise_time_ns();
            println!("Pop {} spent {}ns.", $name, after-before);
            value
        }
    }
}


#[test]
fn it_works() {
    stack!("all", 3+4);
}