//use std::env;
use std::fs;



fn main() {

    let scale = 1;
    let i32brightness: i32;
    let file_path = "/sys/bus/iio/devices/iio:device0/in_illuminance_raw";
    let path_to_brightness = "/sys/class/leds/chromeos::kbd_backlight/brightness";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lumen = contents.trim().parse::<i32>().unwrap();

    if lumen > 70 {
        i32brightness = 0;
    } 
    else if lumen <= 0 {
        i32brightness = 100;
    }else {
        i32brightness = lumen * scale;
    }

    let brightness = &i32brightness.to_string();
    
    println!("{lumen}, {brightness}");

    let res = fs::write(path_to_brightness, brightness);

    match res {
        Err(e) => println!("Fuckin Error {e}"),
        Ok(f) => f,
    }

}
