/*
 * #include <fcntl.h>
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
use libc;
use std::{
    fs::OpenOptions,
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
};
use nix::ioctl_readwrite_bad;

// #[repr(C)]
// pub struct payload {
//     trigger_global_refresh: bool,
// }

// number comes from a c printf(%lu, ....)
const DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH: u64 = 3221316672;
ioctl_readwrite_bad!(ebc_ioctl, DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH, libc::c_uchar);

pub fn trigger_global_refresh() {
    println!("Hello, world!");
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
}
