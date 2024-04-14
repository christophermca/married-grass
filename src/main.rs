// use tokio;
// use zbus::Connection;
// use std::future::Future;
use dbus::blocking::Connection;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;

    let proxy = conn.with_proxy(
        "org.freedesktop.appearance",
        "/org/freedesktop/portal/desktop",
        Duration::from_millis(5000),
    );
    print("proxy keys", proxy.keys())
    proxy.get("org.freedesktop.appearance", "Metadata")?;
    return OK(());
}
