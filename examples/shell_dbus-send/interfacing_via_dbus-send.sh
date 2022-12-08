#!/usr/bin/env sh

# Controlling the EBC driver

# trigger a global refresh
dbus-send --system --print-reply --dest=org.pinenote.ebc / org.pinenote.ebc.TriggerGlobalRefresh

# set the waveform. Valid valus: 0-7
dbus-send --system --print-reply --dest=org.pinenote.ebc / org.pinenote.ebc.SetDefaultWaveform byte:2

# retrieve current waveform
dbus-send --print-reply --system --dest=org.pinenote.ebc / org.pinenote.ebc.GetDefaultWaveform
