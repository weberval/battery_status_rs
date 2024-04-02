#[macro_export]
macro_rules! main {
    ($feature : ty) => {
        use battery_status_action::Action;

        let arguments: arguments::Arguments =
            arguments::parse(std::env::args()).expect("failed to parse command line arguments");

        let threshold: i32 = arguments.get::<i32>("threshold").unwrap_or(15i32);

        let mut action: $feature = battery_status_action::Action::new(threshold);

        let _ = sd_notify::notify(true, &[sd_notify::NotifyState::Ready]);

        loop {
            let _ = sd_notify::notify(true, &[sd_notify::NotifyState::Watchdog]);
            action.refresh();
            std::thread::sleep(std::time::Duration::from_secs(60))
        }
    };
}