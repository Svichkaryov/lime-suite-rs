use lime_suite::device::info::get_library_version;
use lime_suite::device::Context;

fn main() {
    let lib_version =
        get_library_version().expect("failed to retrieve library version");
    println!("Library version: {:#?}\n", lib_version);

    // Find devices
    let dev_list = Context::get_device_list(true)
        .expect("failed to retrieve device list");

    println!("Device list: {:#?}\n", dev_list);

    if dev_list.is_empty() {
        return;
    }

    // Open device
    // let dev = Context::open(Some(dev_list[0].clone())).unwrap();

    println!("Open the first available device");
    let dev = Context::open(None).expect("failed to open device");

    let dev_info = dev
        .get_device_info()
        .expect("failed to retrieve device info");
    println!("Device info: {:#?}\n", dev_info);

    let program_modes = dev
        .get_program_modes()
        .expect("failed to retrive program modes");
    println!(
        "List of supported programming modes: {:#?}\n",
        program_modes
    );

    // Manual closing of the device
    dev.close()
        .map_err(|(_, err)| err)
        .expect("failed to close device");
}
