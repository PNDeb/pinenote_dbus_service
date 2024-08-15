#!/usr/bin/env sh
pwd_save=$PWD
outdir="package_pinenote_dbus_service"
test -d "${outdir}" && rm -r "${outdir}"
mkdir "${outdir}"
cd "${outdir}"
debcargo package \
        --changelog-ready \
        --config "${pwd_save}/"pns_debcargo.toml \
        pinenote_dbus_service

