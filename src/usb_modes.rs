use std::process::Command;

// NOTE:
// see if this could be a better way to interact with systemd
// https://crates.io/crates/libsystemd

pub fn activate_mtp_gadget() {
    println!("activate_mtp_gadget");

    let output = Command::new("sh")
            .arg("-c")
            .arg("systemctl status usb_mtp.service")
            .output()
            .expect("failed to execute process");

    let p_stdout = output.stdout;

    println!("CMD OUTPUT: {}", String::from_utf8(p_stdout).unwrap());

    let output = Command::new("sh")
            .arg("-c")
            .arg("systemctl start usb_mtp.service")
            .output()
            .expect("failed to execute process");

    let p_stdout = output.stdout;
    println!("{:?}", p_stdout);

}

pub fn disable_mtp_gadget() {
    println!("activate_mtp_gadget");

    let output = Command::new("sh")
            .arg("-c")
            .arg("systemctl status usb_mtp.service")
            .output()
            .expect("failed to execute process");

    let p_stdout = output.stdout;

    println!("CMD OUTPUT: {}", String::from_utf8(p_stdout).unwrap());

    let output = Command::new("sh")
            .arg("-c")
            .arg("systemctl stop usb_mtp.service")
            .output()
            .expect("failed to execute process");

    let p_stdout = output.stdout;
    println!("{:?}", p_stdout);
}
