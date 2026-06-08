use lime_suite::device::Context;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Find devices
    let dev_list = Context::get_device_list(true)
        .expect("failed to retrieve device list");

    println!("Device list: {:#?}\n", dev_list);

    if dev_list.is_empty() {
        return;
    }

    println!("Open the first available device");
    let dev = Context::open(None).expect("failed to open device");

    let dev_info = dev
        .get_device_info()
        .expect("failed to retrieve device info");
    println!("Device info: {:#?}\n", dev_info);

    let config_name = match args.len() {
        1 => match dev_info {
            Some(info) => {
                format!("{}_chip_config.ini", info.device_name)
            }
            None => String::from("chip_config.ini"),
        },
        _ => String::from(&args[1]),
    };

    println!("Saving LMS chip configuration ({})", config_name);
    dev.save_config(config_name.as_str())
        .expect("failed to save config");

    // Manual closing of the device
    dev.close()
        .map_err(|(_, err)| err)
        .expect("failed to close device");
}
