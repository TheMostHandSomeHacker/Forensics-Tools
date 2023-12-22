use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


pub fn stdout_init() -> StandardStream {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
}

pub fn stdout_close(stdout: &mut StandardStream){
    stdout.reset().unwrap();
}

pub fn warning_message(msg: &str, stdout: &mut StandardStream) {
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    println!("\t{}", msg);
    stdout_close(stdout);
}

pub fn cmc_showup(stdout: &mut StandardStream) {
    let msg1 = "
                      **********************************************
                      *                                            *
                      *        EDWARD FORENSIC - Version 1.0.0     *
                      *                                            *
                      **********************************************";

    let msg6 = "Author: Tien Cong Nguyen";

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
    println!("\n{}\n\n", msg6);
    println!("{}\n\n", msg1);

    //stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta))).unwrap();
    
    stdout_close(stdout);
    
}

pub fn cve_checking() {
    let msg = "[+] CVE Checking ... ";
    println!("{}", msg);
}