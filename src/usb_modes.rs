use std::process::Command;

// NOTE:
// see if this could be a better way to interact with systemd
// https://crates.io/crates/libsystemd

pub fn activate_mtp_gadget() {
    println!("activate_mtp_gadget");

    let output = Command::new("sh")
            .arg("-c")
            .arg("systemctl status ebc_gadget_mtp_mode.service")
            .output()
            .expect("failed to execute process");

    let p_stdout = output.stdout;
    println!("{:?}", p_stdout);

}
