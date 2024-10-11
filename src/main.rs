extern crate nvml_wrapper;
use nvml_wrapper::{enum_wrappers::device::{Clock, TemperatureSensor}, Device, Nvml};

fn main() -> Result<(), nvml_wrapper::error::NvmlError> {
    loop {
        let nvml = Nvml::init()?;
        // Get the first `Device` (GPU) in the system
        let device:Device = nvml.device_by_index(0)?;
         
        // base info
        let brand = device.brand()?; // GeForce on my system
        let fan_speed_1 = device.fan_speed(0)?; // Currently 17% on my system
        let fan_speed_2 = device.fan_speed(1)?; // Currently 17% on my system
        let memory_info = device.memory_info()?; // Currently 1.63/6.37 GB used on my system
        let power_limit = device.enforced_power_limit()?; // 275k milliwatts on my system
        // let power_draw = device.power_usage()?; // 30W on my system
    
        // read temperature
        let sensor_gpu: TemperatureSensor = TemperatureSensor::Gpu;
        let temp_board = device.temperature(sensor_gpu);
        // clock types
        let clock = Clock::Graphics;
        let clock2 = Clock::Graphics;
        let memory_clock = Clock::Memory;
        let memory_clock2 = Clock::Memory;
        
        // read clock speeds
        let gpu_max_clock = device.max_clock_info(clock)?;
        let gpu_current_clock = device.clock_info(clock2);
        let memory_max_clock = device.max_clock_info(memory_clock)?;
        let memory_current_clock = device.clock_info(memory_clock2);
    
        println!("Brand: {:?}", brand);
        println!("Fan 1 Speed: {:?}%", fan_speed_1);
        println!("Fan 2 Speed: {:?}%", fan_speed_2);
        println!("Temperature Board: {:?}", temp_board); // 30°C on my system
        println!("GPU Max Clock: {:?}", gpu_max_clock);
        println!("GPU Current Clock: {:?}", gpu_current_clock);
        println!("Power Limit: {:?}", power_limit);
        // println!("Power Draw: {:?}", power_draw);
        println!("Memory Max Clock: {:?}", memory_max_clock);
        println!("Memory Current Clock: {:?}", memory_current_clock);
        println!("vRam Info: {:?}", memory_info);
        println!("-----------------------------------");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

}
