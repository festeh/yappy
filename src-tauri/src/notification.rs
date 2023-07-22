use notify_rust::Notification;

pub fn send_notification(s: &str) {
    Notification::new()
        .summary(s)
        .show()
        .expect("Failed to send notification");
}
