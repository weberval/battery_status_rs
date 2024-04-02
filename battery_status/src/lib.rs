pub struct BatteryStatus {
    battery_manager: starship_battery::Manager,
}

impl BatteryStatus {
    pub fn new() -> BatteryStatus {
        BatteryStatus {
            battery_manager: starship_battery::Manager::new()
                .expect("Failed to create battery manager"),
        }
    }

    fn batteries(&self) -> starship_battery::Batteries {
        return self
            .battery_manager
            .batteries()
            .expect("failed to get battery iterator");
    }

    pub fn is_power_supply_online(&self) -> bool {
        for battery_result in self.batteries() {
            let battery = battery_result.expect("failed to get battery");
            if battery.state() == starship_battery::State::Charging {
                println!("power supply is online!");
                return true;
            }
        }
        return false;
    }

    fn get_energy_full(&self) -> f32 {
        let mut energy_full: f32 = 0f32;
        for battery in self.batteries() {
            energy_full += battery.expect("failed to get battery").energy_full().value
        }
        return energy_full;
    }

    fn get_energy(&self) -> f32 {
        let mut energy: f32 = 0f32;
        for battery in self.batteries() {
            energy += battery.expect("failed to get battery").energy().value
        }
        return energy;
    }

    pub fn get_percentage(&self) -> i32 {
        let energy_full = self.get_energy_full();
        let energy = self.get_energy();
        let percentage: i32 = ((energy / energy_full) * 100f32).round() as i32;
        #[cfg(debug_assertions)]
        {
            println!("battery is at {:#?}%", percentage);
        }
        return percentage;
    }
}