use std::time::Duration;

use zbus::Connection;

#[derive(Clone, Debug)]
pub struct DBus {
    pub conn: Connection,
}

impl Default for DBus {
    fn default() -> Self {
        DBus::new()
    }
}

impl DBus {
    pub fn new() -> Self {
        let conn = async_std::task::block_on(Connection::session()).unwrap();
        Self { conn }
    }

    pub fn send(&self, time: String) {
        async_std::task::spawn(async {
            let t = time;
            let conn = Connection::session().await.unwrap();

            let res = conn.call_method(
                Some("i3.status.rs"),
                "/Pomodoro",
                Some("i3.status.rs"),
                "SetStatus",
                &t,
            );
            let dur = Duration::from_millis(50);
            let _ = async_std::future::timeout(dur, res).await;
        });
        // .expect("Failed to send message");
    }
}
