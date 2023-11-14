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
 * dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.PenDoScan
 * dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.PenSetAddress string:"ta:19:41:03:34:2b"
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


fn main() -> Result<(), Box<dyn Error>> {
    // Let's start by starting up a connection to the session bus and request a name.
    let c = Connection::new_system()?;
    c.request_name("org.pinenote.ebc", false, true, false)?;
    c.request_name("org.pinenote.pen", false, true, false)?;
    c.request_name("org.pinenote.usb", false, true, false)?;

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
        b.method(
            "PenSetAddress",
            ("pen_address", ),
            ( ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, (pen_address, ): (String, ) | {
                println!("Initiating scan");
                sys_handler::write_to_file(
                    "/sys/devices/platform/spi-gpio/spi_master/spi4/spi4.0/pen_address",
                    &pen_address.to_owned()
                );
                println!("pen address set to {}", pen_address);

                Ok(())
            }
        );
        b.method(
            "PenDoScan",
            (),
            ("scan_results", ),
            move |_ctx: &mut Context, _dum: &mut EbcObject, ( )| {
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
                println!("Got results: |{}|", scan_results);

                Ok((scan_results, ))
            }
        );
    });

    let iface_token3 = cr.register("org.pinenote.usb", |b| {
        // we use this signal to notify users of a cable connection
        let usb_cable_connected = b.signal::<( ), _>("USBCableConnected", ()).msg_fn();

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



    });

    /* We need:
     *  activate usb-mtp
     *  activate usb-network
     *  activate usb-tablet mode
     *  maybe: reset charge mode?
     *
     * */

    // Let's add the "/" path, which implements the com.example.dbustest interface,
    // to the crossroads instance.
    cr.insert("/ebc", &[iface_token], EbcObject{});
    cr.insert("/pen", &[iface_token2], EbcObject{});
    cr.insert("/usb", &[iface_token3], EbcObject{});

    // Serve clients forever.
    println!("Starting PineNote DBUS service");
    cr.serve(&c)?;
    unreachable!()
}
