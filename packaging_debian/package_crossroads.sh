#!/usr/bin/env sh
pwd_save=$PWD

outdir="out_crossroads"
test -d "${outdir}" && rm -r "${outdir}"
mkdir "${outdir}"
cd "${outdir}"
debcargo package \
	--changelog-ready \
	--config "${pwd_save}/"crossroads_debcargo.toml dbus-crossroads

cd rust-dbus-crossroads*/
dpkg-buildpackage -us -uc
