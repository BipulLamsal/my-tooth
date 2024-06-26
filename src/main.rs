use blurz::{BluetoothAdapter, BluetoothDevice};
use dbus::blocking::Connection;
use std::time::Duration;

const DEVICE_MAC: &str = "dev_6E_EE_82_0F_45_86";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = true;
    println!("Daemon Running!");
    loop {
        let bluetooth_device = BluetoothDevice::new(format!("/org/bluez/hci0/{}", DEVICE_MAC));
        let bluetooth_adapter = BluetoothAdapter::init()?;
        if bluetooth_device.is_connected()? {
            controller = true;
        }
        if !bluetooth_device.is_connected()? && controller {
            pause_audio_process()?;
            println!("Audio Process Paused");
            controller = false;
        }
        if !bluetooth_adapter.is_powered()? && controller {
            pause_audio_process()?;
            controller = false
        }
    }
}

fn pause_audio_process() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let dbus_proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    let (names,): (Vec<String>,) =
        dbus_proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;
    for name in names {
        if name.contains("org.mpris.MediaPlayer2") {
            println!("{:?}", name);
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
