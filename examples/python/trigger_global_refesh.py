#!/usr/bin/env python
from pydbus import SystemBus

system_bus = SystemBus()
proxy = system_bus.get('org.pinenote.ebc', '/')
# get interface help, not strictly required
help(proxy)

proxy.TriggerGlobalRefresh()
