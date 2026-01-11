use sh::smart_device::Thermometer;

fn main() {
    let mut celsius_thermometer = sh::smart_device::CelsiusThermometer::<f32>::new();

    println!(
        "Current temperature: {}",
        celsius_thermometer.get_temperature()
    );
}
