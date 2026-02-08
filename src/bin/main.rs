use sh::smart_device::Socket;
use sh::smart_device::Thermometer;

fn main() {
    let mut celsius_thermometer =
        sh::smart_device::CelsiusThermometer::<f32>::new(25.0, -10.0, 10.0);
    println!(
        "Current temperature: {}",
        celsius_thermometer.get_temperature()
    );

    let mut power_socket = sh::smart_device::PowerSocket::<f32>::new(100.0, 25.0);
    println!("Current power: {}", power_socket.get_power());
    power_socket.turn_on();
    println!("Current power: {}", power_socket.get_power());
    power_socket.turn_off();
    println!("Current power: {}", power_socket.get_power());
    power_socket.turn_on();
    println!("Current power: {}", power_socket.get_power());
}
