//use std::env;
use std::fs;



fn main() {

    let file_path = "/sys/bus/iio/devices/iio:device0/in_illuminance_sampling_frequency";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let path_to_brightness = "/sys/class/leds/chromeos::kbd_backlight/brightness";

    let res = fs::write(path_to_brightness, "100");

    match res {
        Err(e) => println!("Fuckin Error {e}"),
        Ok(f) => f,
    }

    println!("{contents}");

}
