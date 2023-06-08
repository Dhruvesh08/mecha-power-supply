use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct PowerSupply {
    name: String,
    r#type: String,
    status: String,
    present: bool,
    voltage_now: u32,
    current_now: i32,
    capacity: u8,
    capacity_level: String,
    temp: i32,
    technology: String,
    charge_full: u32,
    charge_now: u32,
    charge_full_design: u32,
    manufacturer: String,
}

impl PowerSupply {
    fn new() -> Self {
        Self {
            name: String::new(),
            r#type: String::new(),
            status: String::new(),
            present: false,
            voltage_now: 0,
            current_now: 0,
            capacity: 0,
            capacity_level: String::new(),
            temp: 0,
            technology: String::new(),
            charge_full: 0,
            charge_now: 0,
            charge_full_design: 0,
            manufacturer: String::new(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("/sys/class/power_supply/bq27441-0/uevent")?;
    let reader = BufReader::new(file);

    let mut power_supply = PowerSupply::new();

    for line in reader.lines() {
        let line = line?;
        let parts = line.split('=').collect::<Vec<_>>();

        match parts[0] {
            "POWER_SUPPLY_NAME" => power_supply.name = parts[1].to_string(),
            "POWER_SUPPLY_TYPE" => power_supply.r#type = parts[1].to_string(),
            "POWER_SUPPLY_STATUS" => power_supply.status = parts[1].to_string(),
            "POWER_SUPPLY_PRESENT" => power_supply.present = parts[1] == "1",
            "POWER_SUPPLY_VOLTAGE_NOW" => power_supply.voltage_now = parts[1].parse().unwrap_or_default(),
            "POWER_SUPPLY_CURRENT_NOW" => power_supply.current_now = parts[1].parse().unwrap_or_default(),
            "POWER_SUPPLY_CAPACITY" => power_supply.capacity = parts[1].parse().unwrap_or_default(),
            "POWER_SUPPLY_CAPACITY_LEVEL" => power_supply.capacity_level = parts[1].to_string(),
            "POWER_SUPPLY_TEMP" => power_supply.temp = parts[1].parse().unwrap_or_default(),
            "POWER_SUPPLY_TECHNOLOGY" => power_supply.technology = parts[1].to_string(),
            "POWER_SUPPLY_CHARGE_FULL" => power_supply.charge_full = parts[1].parse().unwrap_or_default(),
            "POWER_SUPPLY_CHARGE_NOW" => power_supply.charge_now = parts[1].parse().unwrap_or_default(),
            "POWER_SUPPLY_CHARGE_FULL_DESIGN" => power_supply.charge_full_design = parts[1].parse().unwrap_or_default(),
            "POWER_SUPPLY_MANUFACTURER" => power_supply.manufacturer = parts[1].to_string(),
            _ => (),
        }
    }

    println!("{:#?}", power_supply);

    Ok(())
}
