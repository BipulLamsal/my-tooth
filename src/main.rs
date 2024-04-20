use blurz::{BluetoothAdapter, BluetoothDevice};
// use daemonize::Daemonize;
use dbus::blocking::Connection;

use std::{fs::File, time::Duration};
const DEVICE_MAC: &str = "dev_6E_EE_82_0F_45_86";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let stdout = File::create("/tmp/mytooth.out")?;
    // let stderr = File::create("/tmp/mytooth.err")?;
    let adapter = BluetoothAdapter::init()?;
    let mut controller = true;
    while adapter.is_powered()? {
        let bluetooth_device = BluetoothDevice::new(format!("/org/bluez/hci0/{}", DEVICE_MAC));
        if !bluetooth_device.is_connected()? && controller {
            pause_audio_process()?;
            controller = false;
        }
        if bluetooth_device.is_connected()? {
            controller = true;
        }
    }
    Ok(())
}

fn pause_audio_process() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let dbus_proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    let (names,): (Vec<String>,) =
        dbus_proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;
    for name in names {
        if name.contains("org.mpris.MediaPlayer2") {
            let proxy = conn.with_proxy(
                &name,
                "/org/mpris/MediaPlayer2",
                Duration::from_millis(5000),
            );
            proxy.method_call("org.mpris.MediaPlayer2.Player", "Pause", ())?;
        }
    }
    Ok(())
}
