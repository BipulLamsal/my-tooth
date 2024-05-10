## BLE Daemon Service 
A simple daemon service that pauses audio services when bluetooth disconnects using dbus proxy.
It basically signals mpris dbus interface to pause when BLE device disconnects.
*requires mac address of the device*
## Service Config as User
(most preffered)
service config location: `$HOME/.config/systemd/user/mytooth.service`
bin executable location: `/usr/local/bin/mytooth`
```
[Unit]
Description=Simple service to pause the audio services when disconnected

[Service]
Type=simple
ExecStart=/usr/local/bin/mytooth

[Install]
WantedBy=default.target #init service for user session
```
### Enabling service for session service
uses `systemctl` to manage systemd service
```
systemctl --user daemon-reload
systemctl --user enable mytooth # runs on startup
systemctl --user start mytooth
systemctl --user stop mytooth # disable the service
```

## Service Config as System
(might fail on startup because dbus socket is not established before session starts)
service file config location:`/etc/systemd/system/mytooth.service`
bin executable location: `/bin/mytooth/mytooth`
```
[Unit]
Description=Simple service to pause the audio services when disconnected
After=bluetooth.target dbus.target dbus.socket
Requires=dbus.socket

[Service]
User=simphone
Type=simple
WorkingDirectory=/bin/mytooth
ExecStart=/bin/mytooth/mytooth
Environment="Display=:1"
Environment="DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus"
RestartSec=5s

[Install]
WantedBy=multi-user.target 
```
### Enabling service for system service
uses `systemctl` to manage systemd service
```
sudo systemctl daemon-reload
sudo systemctl enable mytooth # runs on startup
sudo systemctl start mytooth
sudo systemctl stop mytooth # disable the service
```

#### Resources
[dbus documentation](https://www.freedesktop.org/wiki/Software/dbus/) 
[MPRIS D-Bus Interface Specification](https://specifications.freedesktop.org/mpris-spec/latest/)
[Bluez D-bus tutorial](https://ukbaz.github.io/howto/python_gio_1.html)
