# Pinenote DBus Service

A simple DBus service that exposes various controls and settings related to the
Pine64 Pinenote via system-wide DBus interface. The service is written in rust
and uses the dbus-rs crate (https://github.com/diwic/dbus-rs).

> **Warning**
> This is my very first rust program and my very first DBus experience.
> Expect bugs and basically every beginners mistake related to rust and DBus
> ;-) I would be very grateful to any bug reports and style/programming
> suggestions!

# Requirements and target platform

This program is intended solely to run on the Pine64 Pinenote (arm64) and
requires the ebc driver modifications maintained in this repository:

https://github.com/m-weigand/linux

A ready-to-use kernel branch for the Pinenote is, for example:

https://github.com/m-weigand/linux/tree/branch_pinenote_6-12_v1

# Compilation

Compile with:

	cargo build

Create a Debian package using cargo-deb:

	cargo deb

# Installation

Just calling the generated binary should suffice to register the dbus service
on org.pinenote.* using the system bus:

* **org.pinenote.ebc** /ebc for ebc driver control
* **org.pinenote.pen** /pen for pen button driver control

A systemd unit is also supplied in the **systemd_units/** subdirectory and
should be the preferred way to start the service.

When generating and using the Debian package the systemd unit is automatically
installed, enabled and started.

DBus services on the system bus are subject to security restrictions and
require explicit permission setting. This was attempted using the configuration
file **dbus_security_configuration/pinenote.conf**, which should be placed in
here **/etc/dbus-1/system.d/pinenote.conf** (done automatically by the Debian
package).

# Usage

Please refer to the **examples/** subdirectory for usage examples in various
languages (shell/dbus-send, python, gjs, rust).

For the impatient, here are a few dbus-send commands:

	# trigger full refresh
	dbus-send --system --print-reply --dest=org.pinenote.ebc /ebc org.pinenote.ebc.TriggerGlobalRefresh
	# set and get waveform used
	dbus-send --system --print-reply --dest=org.pinenote.ebc /ebc org.pinenote.ebc.SetDefaultWaveform byte:2
	dbus-send --print-reply --system --dest=org.pinenote.ebc /ebc org.pinenote.ebc.GetDefaultWaveform

	# initiate a scan for the stylus (buttons). Repeatedly push the buttons
	# during the 12-20 seconds scan interval
	dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.DoScan
	dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.AutoConnect
	# connect to the mac address retrieved by the PenDoScan command
	dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.SetAddress string:"12:23:45:56:90:4b"
	dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.ForgetAddress
	dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.GetAddress
	dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.GetVersion
	dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.pinenote.pen.GetBattery

    # travel mode
    dbus-send --print-reply --system --dest=org.pinenote.misc /misc org.pinenote.misc.EnableTravelMode
    dbus-send --print-reply --system --dest=org.pinenote.misc /misc org.pinenote.misc.DisableTravelMode
	dbus-send --print-reply --system --dest=org.pinenote.misc /misc org.pinenote.misc.GetTravelMode

    # set off-screen content (temporary, not persistent across reboots)
    dbus-send --print-reply --system --dest=org.pinenote.ebc /ebc org.pinenote.ebc.SetOfflineScreenFromFileTemporary string:"/lib/firmware/rockchip/rockchip_ebc_default_screen.bin"

## Introspection

    dbus-send --print-reply --system --dest=org.pinenote.ebc /ebc org.freedesktop.DBus.Introspectable.Introspect
    dbus-send --print-reply --system --dest=org.pinenote.pen /pen org.freedesktop.DBus.Introspectable.Introspect
    dbus-send --print-reply --system --dest=org.pinenote.usb /usb org.freedesktop.DBus.Introspectable.Introspect
    dbus-send --print-reply --system --dest=org.pinenote.misc /misc org.freedesktop.DBus.Introspectable.Introspect

# Some design choices that probably need revising

* At this point getter/setter functions are mostly implemented using dbus
  methods instead of using dbus properties (some are implement in both ways).
  While I see the benefits of the properties, I find using them a little bit
  awkward, especially introspecting them seems complicated. Need to look into
  this again...
* At the moment changing the waveform and changing the bw_mode parameter both
  emit the "WaveformChanged" signal. This does not fit. However, I would like
  to somehow prevent duplicate signals to be emitted. For example, at the
  moment I'm triggering both one global refresh for changing the waveform and
  for changing the bw_mode, leading to two refreshes when switching from
  grayscale to black/white+A2-waveform mode. Probably some kind of combined
  getter/setter-method+associated signal is the solution here.

# Debian Packaging

## The hard way: Via debcargo

* git clone https://github.com/PNDeb/pinenote_dbus_service.git
* cd pinenote_dbus_service
* cd packaging_debian/
* Packaging dbus-crossroads
  (https://github.com/diwic/dbus-rs/blob/master/dbus-crossroads/), as of this
  writing not part of Debian trixie:
   * Check `cat overlay_crossroads/changelog` and adapt
   * Execute:

         bash package_crossroads.sh
         # we need crossroads installed in order to build the dbus service
         dpkg -i out_crossroads/*.deb

* Packaging pinenote-dbus-service:
	* (Important!) Check and adapt: *overlay_pinenote_dbus_service/changelog*
	* Execute:

          package_pinenote_dbus_service.sh

# License/Copying

The main.rs file started from this dbus-rs example:
Based upon https://github.com/diwic/dbus-rs/blob/master/dbus-crossroads/examples/server_cr.rs
Unless otherwise noted in the header of a given file, the code is Apache 2.0 /
MIT dual licensed, following the licensing scheme of the dbus-rs project.

Copyright of the dbus-rs project: (c) 2014-2018 David Henningsson
<diwic@ubuntu.com> and other contributors

Copyright of the rest: 2022-2024 Maximilian Weigand <mweigand@mweigand.net>
