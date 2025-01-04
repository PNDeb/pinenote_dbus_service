use std::{
    fs::OpenOptions,
    // os::unix::{fs::OpenOptionsExt, io::AsRawFd},
    io::Write,
    io::Read,
};
use std::io::{BufRead, BufReader};

pub fn write_to_file(filename : &str, new_value : &str) {
    let file = OpenOptions::new().write(true)
         .open(filename).expect("Error opening the file");

    write!(&file, "{}", new_value).unwrap();
}

pub fn read_file(filename : &str) -> String {
    let file = OpenOptions::new().read(true)
         .open(filename).expect("Error opening the file");
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut num_bytes = 1;
    while num_bytes > 0 {
        num_bytes = reader.read_line(&mut buf).unwrap_or_default();
    }
    //println!("buf: @{}@", buf);

    return buf.trim_end().to_string();
}

fn read_ebc_file(parameter : &str) -> String {
    let parameter_file = format!("/sys/module/rockchip_ebc/parameters/{parameter}");
    let file = OpenOptions::new().read(true)
         .open(parameter_file).expect("Error opening the file");
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let _num_bytes = reader.read_line(&mut buf).unwrap();

    return buf.trim_end().to_string();
}

fn write_ebc_file(parameter : &str, new_value : u8) {
    let device = format!("/sys/module/rockchip_ebc/parameters/{parameter}");
    let file = OpenOptions::new().write(true)
         .open(device).expect("Error opening the file");

    write!(&file, "{}", new_value).unwrap();
}

fn write_ebc_file_u32(parameter : &str, new_value : u32) {
    let device = format!("/sys/module/rockchip_ebc/parameters/{parameter}");
    let file = OpenOptions::new().write(true)
         .open(device).expect("Error opening the file");

    write!(&file, "{}", new_value).unwrap();
}

pub fn write_ebc_energy_control(new_value : &str) {
    // write: allow only values "on" and "auto"
    let device = format!("/sys/devices/platform/fdec0000.ebc/power/control");
    let file = OpenOptions::new().write(true)
         .open(device).expect("Error opening the file");

    write!(&file, "{}", new_value).unwrap();
}

pub fn read_ebc_energy_control() -> String {
    let parameter_file = format!("/sys/devices/platform/fdec0000.ebc/power/control");
    let file = OpenOptions::new().read(true)
         .open(parameter_file).expect("Error opening the file");
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let _num_bytes = reader.read_line(&mut buf).unwrap();

    return buf.trim_end().to_string();
}

/************************************************************************/

pub fn set_auto_refresh(state: bool) {
    write_ebc_file("auto_refresh", state as u8);
    // let ebc_device = "/sys/module/rockchip_ebc/parameters/auto_refresh";
    // let file = OpenOptions::new().write(true)
    //      .open(ebc_device).expect("Error opening the file");

    // write!(&file, "{}", state).unwrap();
}

pub fn get_auto_refresh() -> bool{
    let ebc_device = "/sys/module/rockchip_ebc/parameters/auto_refresh";
    let mut file = OpenOptions::new().read(true)
         .open(ebc_device).expect("Error opening the file");

    let mut state = [0; 1];
    // let _ = file.by_ref().take(8).read(&mut state);
    let _ = std::io::Write::by_ref(&mut file).take(8).read(&mut state);

    // state[0] = 0;
    // let read_result = file.read_exact(&mut state).expect("Reading failed");
    // // state as bool
    // true
    state[0] != 0
}

pub fn get_bw_dither_invert() -> u8{
    read_ebc_file("bw_dither_invert").parse::<u8>().unwrap()
}

pub fn set_bw_dither_invert(new_mode: u8){
    write_ebc_file("bw_dither_invert", new_mode);
}

pub fn get_delay_a() -> u32{
    read_ebc_file("delay_a").parse::<u32>().unwrap()
}

pub fn set_delay_a(new_mode: u32){
    write_ebc_file_u32("delay_a", new_mode);
}

pub fn get_split_area_limit() -> u32{
    read_ebc_file("split_area_limit").parse::<u32>().unwrap()
}

pub fn set_split_area_limit(new_mode: u32){
    write_ebc_file_u32("split_area_limit", new_mode);
}

pub fn set_default_waveform(waveform: u8) {
    let ebc_device = "/sys/module/rockchip_ebc/parameters/default_waveform";
    let file = OpenOptions::new().read(true) .write(true)
         .open(ebc_device).expect("Error opening the file");

    write!(&file, "{}", waveform).unwrap();
}

pub fn get_default_waveform() -> u8{
   read_ebc_file("default_waveform").parse::<u8>().unwrap()
}

pub fn get_bw_mode() -> u8{
    read_ebc_file("bw_mode").parse::<u8>().unwrap()
}

pub fn set_bw_mode(new_mode: u8){
    write_ebc_file("bw_mode", new_mode);
}

pub fn get_no_off_screen() -> u8{
    read_ebc_file("no_off_screen").parse::<u8>().unwrap()
}

pub fn set_no_off_screen(new_mode: u8){
    write_ebc_file("no_off_screen", new_mode);
}

pub fn get_dclk_select() ->u8 {
    read_ebc_file("dclk_select").parse::<u8>().unwrap()
}

pub fn set_dclk_select(new_mode: u8){
    // todo: allowed values: -1, 0, 1
    write_ebc_file("dclk_select", new_mode);
}

/*
* [x] auto_refresh
* [x] bw_dither_invert
* [ ] bw_mode
* [ ] bw_threshold
* [x] dclk_select
* [x] default_waveform
* [x] delay_a
* [ ] delay_b
* [ ] delay_c
* [ ] diff_mode
* [ ] direct_mode
* [ ] fourtone_hi_threshold
* [ ] fourtone_low_threshold
* [ ] fourtone_mid_threshold
* [ ] hskew_override
* [ ] limit_fb_blits
* [x] no_off_screen
* [ ] panel_reflection
* [ ] prepare_prev_before_a2
* [ ] refresh_threshold
* [ ] refresh_waveform
* [ ] skip_reset
* [x] split_area_limit
* [ ] temp_override
*/
