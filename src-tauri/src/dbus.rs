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

    pub fn send(&self, time: &str) {
        async_std::task::block_on(
            self.conn
                .call_method(
                    Some("i3.status.rs"),
                    "/Pomodoro",
                    Some("i3.status.rs"),
                    "SetStatus",
                    &(time),
                )
        ).expect("Failed to send message");
    }
}

