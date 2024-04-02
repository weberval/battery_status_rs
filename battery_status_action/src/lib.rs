pub struct Notification {
    is_already_notified: bool,
    threshold: i32,
    battery_status: battery_status::BatteryStatus,
}

pub struct Sleep {
    threshold: i32,
    battery_status: battery_status::BatteryStatus,
}

pub trait Action {
    fn new(threshold: i32) -> Self;
    fn refresh(&mut self);
}

impl Action for Notification {
    fn new(threshold: i32) -> Notification {
        Notification {
            is_already_notified: false,
            threshold,
            battery_status: battery_status::BatteryStatus::new(),
        }
    }

    fn refresh(&mut self) {
        if self.battery_status.is_power_supply_online() {
            self.is_already_notified = false;
        } else if !self.is_already_notified {
            let percentage = self.battery_status.get_percentage();
            if percentage < self.threshold {
                match notify_rust::Notification::new()
                    .summary("Battery is running low")
                    .body(&format!("Only {:#?}% left!", percentage))
                    .urgency(notify_rust::Urgency::Critical)
                    .show()
                {
                    Err(_) => eprintln!("failed to display notification"),
                    Ok(_) => {}
                }
                self.is_already_notified = true;
            }
        }
    }
}

impl Action for Sleep {
    fn new(threshold: i32) -> Sleep {
        Sleep {
            threshold,
            battery_status: battery_status::BatteryStatus::new(),
        }
    }

    fn refresh(&mut self) {
        if !self.battery_status.is_power_supply_online() {
            let percentage: i32 = self.battery_status.get_percentage();
            if percentage < self.threshold {
                match system_shutdown::sleep() {
                    Ok(_) => {}
                    Err(_) => eprintln!("failed to put system to sleep"),
                }
            }
        }
    }
}
