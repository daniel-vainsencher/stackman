extern crate time;
use time::precise_time_ns;
pub fn time_ns() -> u64 {
    precise_time_ns()
}

#[macro_export]
macro_rules! stack {
    ($name:expr, $work:expr) => {
        {
            use std::io::Write;
            let mut stderr = std::io::stderr();
            let mut work = {|| $work};
            writeln!(&mut stderr, "Push {}", $name).unwrap();
            let before = $crate::time_ns();
            let value = work();
            let after = $crate::time_ns();
            writeln!(&mut stderr, "Pop {} spent {}ns.", $name, after-before).unwrap();
            value
        }
    }
}


#[test]
fn it_works() {
    stack!("all", 3+4);
}
