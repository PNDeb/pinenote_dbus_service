/*
 * Based upon https://github.com/diwic/dbus-rs/blob/master/dbus-crossroads/examples/server_cr.rs
 * The code is Apache 2.0 / MIT dual licensed, following the licensing scheme of the dbus-rs
 * project.
 * Copyright of the dbus-rs project: (c) 2014-2018 David Henningsson <diwic@ubuntu.com> and other
 * contributors
 *
 *
 *
 * dbus-send --system --print-reply --dest=org.pinenote.ebc /ebc org.pinenote.ebc.TriggerGlobalRefresh
 * dbus-send --system --print-reply --dest=org.pinenote.ebc /ebc org.pinenote.ebc.SetDefaultWaveform byte:2
 * dbus-send --print-reply --system --dest=org.pinenote.ebc /ebc org.pinenote.ebc.GetDefaultWaveform
 *
 * dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.DoScan
 * dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.SetAddress string:"ta:19:41:03:34:2b"
 * */
use dbus::blocking::Connection;
use dbus_crossroads::{Crossroads, Context};
use std::error::Error;

mod ebc_ioctl;
mod sys_handler;
mod usb_modes;

// This is the object that we are going to store inside the crossroads instance and that will be
// provided to all methods
struct EbcObject {
}

// check if a given string is a valid mac address for the BT pen
fn check_mac(mac: String) -> bool {
    let mut check: bool = true;

    if mac.chars().count() != 17 {
        check = false;
    }

    if mac.chars().nth(2) != Some(':') {
        check = false;
    }
    if mac.chars().nth(5) != Some(':') {
        check = false;
    }
    if mac.chars().nth(8) != Some(':') {
        check = false;
    }
    if mac.chars().nth(11) != Some(':') {
        check = false;
    }

    // TODO check the rest for valid hex values

    return check;
}

fn pen_do_scan() -> Vec<String> {
    let mut macs: Vec<String> = Vec::new();

    println!("Initiating scan");
    sys_handler::write_to_file(
        "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/scan",
        "1"
    );
    println!("Done");
    // do we need to wait?
    let scan_results = sys_handler::read_file(
        "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/scan",
    );
    println!("Got results: |{}|. Scanning done", scan_results);
    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    //
    let mut lines = scan_results.lines();

    // We always ignore the first two lines that are returned
    lines.next();
    lines.next();

    // we expect only to macs after that
    for line in lines {
        if check_mac(line.to_string()) {
            println!("MAC seems ok");
            macs.push(line.to_string());
        }
    }
    return macs;
}

fn pen_get_version() -> String {
    let pen_version = sys_handler::read_file(
        "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/pen_version");
    println!("Current pen version is {}", pen_version);
    return pen_version;
}

fn pen_get_battery() -> String {
    let pen_battery = sys_handler::read_file(
        "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/pen_battery");
    println!("Current pen battery level is {}", pen_battery);
    return pen_battery;
}

fn main() -> Result<(), Box<dyn Error>> {
    // Let's start by starting up a connection to the session bus and request a name.
    let c = Connection::new_system()?;
    c.request_name("org.pinenote.ebc", false, true, false)?;
    c.request_name("org.pinenote.pen", false, true, false)?;
    c.request_name("org.pinenote.usb", false, true, false)?;
    c.request_name("org.pinenote.misc", false, true, false)?;

    // Create a new crossroads instance.
    // The instance is configured so that introspection and properties interfaces
    // are added by default on object path additions.
    let mut cr = Crossroads::new();

    // Let's build a new interface, which can be used for "Hello" objects.
    let iface_token = cr.register("org.pinenote.ebc", |b| {
        // // This row advertises (when introspected) that we can send a HelloHappened signal.
        // // We use the single-tuple to say that we have one single argument, named "sender" of type "String".
        // // The msg_fn returns a boxed function, which when called constructs the message to be emitted.
        let waveform_changed = b.signal::<( ), _>("WaveformChanged", ()).msg_fn();
        let bwmode_changed = b.signal::<( ), _>("BwModeChanged", ()).msg_fn();
        let request_quality_or_performance_mode = b.signal::<(u8, ), _>("ReqQualityOrPerformance", ("requested_mode", )).msg_fn();

        // we need setters/getters for:
        // auto_refresh
        // bw_dither_invert
        // [r+w] bw_mode
        // bw_threshold
        // [r+w] default_waveform
        // diff_mode
        // direct_mode
        // limit_fb_blits
        // panel_reflection
        // prepare_prev_before_a2
        // refresh_threshold
        // refresh_waveform
        // skip_reset
        // split_area_limit
        // + one group call to set all
        b.property("default_waveform")
            .get(|_, _obcobj| {
                Ok(
                    sys_handler::get_default_waveform()
                )
            })
            .set(|_, _ebcobj, value| {
                // if value && device.checking {
                //     Err(MethodErr::failed(&"Device currently under check, cannot bring online"))?
                // }
                // device.online = value;
                sys_handler::set_default_waveform(value);
                Ok(Some(0))
            });

        b.method(
            "TriggerGlobalRefresh",
            (),
            (),
            move |_ctx: &mut Context, _hello: &mut EbcObject, ()| {
                ebc_ioctl::trigger_global_refresh();
                Ok(())
            }
        );

        b.method(
            "SetDefaultWaveform",
            ("waveform", ),
            (),
            move |_ctx: &mut Context, _dum: &mut EbcObject, (waveform, ): (u8, )| {
                sys_handler::set_default_waveform(waveform);
                // emit the signal
                let signal_msg = waveform_changed(_ctx.path(), &());
                _ctx.push_msg(signal_msg);

                Ok(())
            }
        );

        b.method(
            "GetDefaultWaveform",
            (),
            ("current_waveform", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( )| {
                let ret_value = sys_handler::get_default_waveform();

                Ok((ret_value, ))
            }
        );

        b.method(
            "SetBwMode",
            ("new_mode", ),
            (),
            move |_ctx: &mut Context, _dum: &mut EbcObject, (new_mode, ): (u8, )| {
                sys_handler::set_bw_mode(new_mode);
                let signal_msg = bwmode_changed(_ctx.path(), &());
                _ctx.push_msg(signal_msg);

                Ok(())
            }
        );

        b.method(
            "GetBwMode",
            (),
            ("current_mode", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( )| {
                let ret_value = sys_handler::get_bw_mode();

                Ok((ret_value, ))
            }
        );

        b.method(
            "SetAutoRefresh",
            ("state", ),
            (),
            move |_ctx: &mut Context, _dum: &mut EbcObject, (state, ): (bool, )| {
                sys_handler::set_auto_refresh(state);

                Ok(())
            }
        );


        b.method(
            "GetAutorefresh",
            (),
            ("state_autorefresh", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( )| {
                let ret_value = sys_handler::get_auto_refresh();

                Ok((ret_value, ))
            }
        );

        b.method(
            "SetDclkSelect",
            ("state", ),
            (),
            move |_ctx: &mut Context, _dum: &mut EbcObject, (state, ): (u8, )| {
                sys_handler::set_dclk_select(state);

                Ok(())
            }
        );

        b.method(
            "GetDclkSelect",
            (),
            ("dclk_select", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( )| {
                let ret_value = sys_handler::get_dclk_select();

                Ok((ret_value, ))
            }
        );

        b.method(
            "RequestQualityOrPerformanceMode",
            ("mode_request", ),
            (),
            move |_ctx: &mut Context, _dum: &mut EbcObject, (mode_request, ): (u8, )| {

                match mode_request{
                    // quality mode
                    0 | 1 => {
                        let signal_msg = request_quality_or_performance_mode(
                            _ctx.path(), &(mode_request, )
                        );
                        _ctx.push_msg(signal_msg);
                    },
                    _ => println!("Got a request for an unknown performance mode"),

                }

                Ok(())
            }
        );

        // set-function for all (work on progress)
        b.method(
            "SetEBCParameters",
            (
                "default_waveform",
                "bw_mode",
            ),
            (),
            move |_ctx: &mut Context, _dum: &mut EbcObject, (default_waveform,  bw_mode): (u8, u8, )| {
                sys_handler::set_default_waveform(default_waveform);
                sys_handler::set_bw_mode(bw_mode);

                Ok(())
            }
        );


    });

    let iface_token2 = cr.register("org.pinenote.pen", |b| {
        b.signal::<( ), _>("PenRegStatusChanged", ());

        b.method(
            "SetAddress",
            ("pen_address", ),
            ( ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, (pen_address, ): (String, ) | {
                println!("Initiating scan");
                sys_handler::write_to_file(
                    "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/pen_address",
                    &pen_address.to_owned()
                );
                println!("pen address set to {}", pen_address);

                let signal_msg = _ctx.make_signal("PenRegStatusChanged", ());
                _ctx.push_msg(signal_msg);

                Ok(())
            }
        );
        b.method(
            "GetAddress",
            ( ),
            ("pen_address", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( ) | {
                println!("Returning current pen address");
                let pen_address = sys_handler::read_file(
                    "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/pen_address");
                println!("Current pen address is {}", pen_address);

                Ok((pen_address, ))
            }
        );
        b.method(
            "GetVersion",
            ( ),
            ("pen_version", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( ) | {
                println!("Returning pen version");
                let pen_version = pen_get_version();

                Ok((pen_version, ))
            }
        );
        b.method(
            "GetBattery",
            ( ),
            ("pen_battery", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( ) | {
                println!("Returning pen battery");
                let pen_battery = pen_get_battery();

                Ok((pen_battery, ))
            }
        );
        b.method(
            "ForgetAddress",
            ( ),
            ( ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( )| {
                println!("Forgetting any registered address");
                sys_handler::write_to_file(
                    "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/pen_address",
                    "00:00:00:00:00:00"
                );

                let signal_msg = _ctx.make_signal("PenRegStatusChanged", ());
                _ctx.push_msg(signal_msg);

                Ok(())
            }
        );
        b.method(
            "IsRegistered",
            ( ),
            ("pen_is_registered", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( ) | {
                println!("Checking if pen is registered");
                let address = sys_handler::read_file(
                    "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/pen_address");

                let mut is_registered = false;

                if address != "00:00:00:00:00:00" {
                    is_registered = true;
                }

                Ok((is_registered, ))
            }
        );
        b.method(
            "DoScan",
            (),
            ("scan_results", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( )| {
                let macs = pen_do_scan();

                Ok((macs, ))
                // Ok((scan_results, ))
                // Ok((vec![(3_u32, 4_i64, 5_u8)],))
                //Ok((vec, ))
            }
        );
        b.method(
            "AutoConnect",
            (),
            ("success", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( )| {
                println!("Auto Connect starting");
                println!("scanning...");
                let macs = pen_do_scan();
                println!("result count: {}", macs.len());
                if macs.len() == 0 {
                    return Ok((false, )) ;
                }
                println!("Setting pen address to: {}", macs[0]);
                // take the first mac and set it
                sys_handler::write_to_file(
                    "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/pen_address",
                    &macs[0].to_owned()
                );
                // try to get a version from the pen
                let pen_version = pen_get_version();

                // for now we emit the signal in any case, not sure if this should be clarified in
                // the future
                let signal_msg = _ctx.make_signal("PenRegStatusChanged", ());
                _ctx.push_msg(signal_msg);

                if pen_version.chars().count() >= 1 {
                    return Ok((true, ));
                }
                else {
                    return Ok((false, ));
                }
            }
        );
    });

    let iface_token3 = cr.register("org.pinenote.usb", |b| {
        // we use this signal to notify users of a cable connection
        // let usb_cable_connected = b.signal::<( ), _>("USBCableConnected", ()).msg_fn();

        // this method is used to signal to the service
        b.method(
            "usb_cable_connected",
            (),
            (),
            move |_ctx: &mut Context, _hello: &mut EbcObject, ()| {
                // ebc_ioctl::trigger_global_refresh();
                println!("usb_cable_connected was called");
                Ok(())
            }
        );

        b.method(
            "usb_gadget_activate_mtp",
            (),
            (),
            move |_ctx: &mut Context, _hello: &mut EbcObject, ()| {
                usb_modes::activate_mtp_gadget();
                Ok(())
            }
        );

        b.method(
            "usb_gadget_disable_mtp",
            (),
            (),
            move |_ctx: &mut Context, _hello: &mut EbcObject, ()| {
                usb_modes::disable_mtp_gadget();
                Ok(())
            }
        );

    });

    let iface_token4 = cr.register("org.pinenote.misc", |b| {

        b.method(
            "EnableTravelMode",
            (),
            (),
            move |_ctx: &mut Context, _hello: &mut EbcObject, ()| {
                println!("Enabling travel mode");

                // disable cover wakeup
                sys_handler::write_to_file(
                    "/sys/devices/platform/gpio-keys/power/wakeup",
                    "disabled"
                );
                Ok(())
            }
        );

        b.method(
            "DisableTravelMode",
            (),
            (),
            move |_ctx: &mut Context, _hello: &mut EbcObject, ()| {
                println!("Disabling travel mode");

                // enable cover wakeup
                sys_handler::write_to_file(
                    "/sys/devices/platform/gpio-keys/power/wakeup",
                    "enabled"
                );
                Ok(())
            }
        );

    });

    // Let's add the "/" path, which implements the com.example.dbustest interface,
    // to the crossroads instance.
    cr.insert("/ebc", &[iface_token], EbcObject{});
    cr.insert("/pen", &[iface_token2], EbcObject{});
    cr.insert("/usb", &[iface_token3], EbcObject{});
    cr.insert("/misc", &[iface_token4], EbcObject{});

    // Serve clients forever.
    println!("Starting PineNote DBUS service");
    cr.serve(&c)?;
    unreachable!()
}
