/* Links:
 * https://dev.to/kgrech/7-ways-to-pass-a-string-between-rust-and-c-4ieb
 * https://doc.rust-lang.org/std/ffi/struct.CString.html
 *
 * */

/*
#include <fcntl.h>
#include <sys/ioctl.h>
#include <drm.h>

typedef int bool;

struct drm_rockchip_ebc_trigger_global_refresh {
	bool trigger_global_refresh;
};

#define DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH  DRM_IOWR(DRM_COMMAND_BASE + 0x00, struct drm_rockchip_ebc_trigger_global_refresh)

int main()
{
	int fd = open("/dev/dri/by-path/platform-fdec0000.ebc-card", DRM_RDWR);
	if(fd < 0) {
		return 1;
	}
	struct drm_rockchip_ebc_trigger_global_refresh arg;
	arg.trigger_global_refresh = 1;
	int ret = ioctl(fd, DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH, &arg);
	return ret;
}


 *
 * */
use std::{
    fs::OpenOptions,
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
};
use nix::ioctl_readwrite_bad;
use std::ffi::CString;

// #[repr(C)]
// pub struct payload {
//     trigger_global_refresh: bool,
// }

// number comes from a c printf(%lu, ....)
// const DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH: u64 = 3221316672;
const DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH: u64 = 3222299712;
ioctl_readwrite_bad!(ebc_ioctl, DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH, libc::c_uchar);

/*
 *  15 struct drm_rockchip_ebc_off_screen {
 *  16     __u64 info1;
 *  17     char * ptr_screen_content;
 *  18 };
 */

#[repr(C)]
pub struct PayloadEbc2 {
    info1: bool,
    ptr_screen_content: *const u8,
}


const DRM_IOCTL_ROCKCHIP_EBC_SET_OFFSCREEN: u64 = 3222299713;
ioctl_readwrite_bad!(ebc_ioctl_2, DRM_IOCTL_ROCKCHIP_EBC_SET_OFFSCREEN, PayloadEbc2);

pub fn set_offline_screen() {

    println!("Setting offline screen");
    // 1314144
    let test_content = vec![100u8; 1314144];
    if test_content.len() != 1314144{
        println!("Input data payload must be of length 1314144");
        return;
    }

    unsafe {
        let str2 = CString::from_vec_with_nul_unchecked(test_content);
        // let str2 = CString::from_vec_with_nul_unchecked(test_content).expect("Could not create CString");

        let mut payload = PayloadEbc2 {
            info1: true,
            ptr_screen_content: str2.as_ptr(),
        };

        let ebc_device = "/dev/dri/by-path/platform-fdec0000.ebc-card";
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(ebc_device).unwrap();

        let arg_ptr: *mut PayloadEbc2 = &mut payload;
        let result = ebc_ioctl_2(file.as_raw_fd(), arg_ptr);
        match result {
            Err(why) => panic!("{:?}", why),
            Ok(ret) => println!("{}", ret),
        }
    }
}


pub fn trigger_global_refresh() {
    println!("Triggering Global Refresh!");
    let ebc_device = "/dev/dri/by-path/platform-fdec0000.ebc-card";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(ebc_device).unwrap();

    let mut arg: u8 = 1;
    let arg_ptr: *mut u8 = &mut arg;
    unsafe{
        let result = ebc_ioctl(file.as_raw_fd(), arg_ptr);
        match result {
            Err(why) => panic!("{:?}", why),
            Ok(ret) => println!("{}", ret),
        }
    }
    // todo: close?
}

fn main() {
    println!("RUST ebc IOCTL tests!");
    // trigger_global_refresh();
    set_offline_screen();
}
