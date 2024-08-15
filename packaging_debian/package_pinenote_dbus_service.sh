#!/usr/bin/env sh
pwd_save=$PWD
outdir="out_pinenote_dbus_service"
test -d "${outdir}" && rm -r "${outdir}"
mkdir "${outdir}"
cd "${outdir}"
debcargo package \
        --changelog-ready \
        --config "${pwd_save}/"pns_debcargo.toml \
        pinenote_dbus_service

cd rust-pinenote-dbus-service*/
dpkg-buildpackage -us -uc
