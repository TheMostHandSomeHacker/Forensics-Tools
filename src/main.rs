extern crate winreg;

mod msg;
mod cve;
mod common;
mod event;

use msg::{cmc_showup, cve_checking, stdout_init, stdout_close};

use crate::msg::warning_message;
use crate::cve::{check_winrar, check_7zip};

fn main() {
    let mut stdout = stdout_init();

    cmc_showup(&mut stdout);

    cve_checking();
    if check_winrar() == 3{
        warning_message("[!] CVE-2023-38831 warning!", &mut stdout);
    }
    if check_7zip() == 3 {
        warning_message("[!] CVE-2023-31102 warning!", &mut stdout);
    }
}
