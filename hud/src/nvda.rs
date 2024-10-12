// File: nvda.rs
use nvml_wrapper::{enum_wrappers::device::{Brand, Clock, TemperatureSensor}, Device, Nvml, error::NvmlError};

#[derive(Debug)]
pub struct NvdaInfo {
    pub brand: Brand,
    pub fan_speed_1: u32,
    pub fan_speed_2: u32,
    pub memory_used: f64,
    pub memory_total: f64,
    pub power_limit: f64,
    pub temp_board: u32,
    pub gpu_max_clock: u32,
    pub gpu_current_clock: u32,
    pub memory_max_clock: u32,
    pub mem_current_clock: u32,
}

pub fn nvda () -> Result<NvdaInfo, NvmlError> { 
    let nvml = Nvml::init()?;
    // Get the first `Device` (GPU) in the system
    let device:Device = nvml.device_by_index(0)?;
     
    // base info
    let brand = device.brand()?; // GeForce on my system
    let fan_speed_1 = device.fan_speed(0)?; // Currently 17% on my system
    let fan_speed_2 = device.fan_speed(1)?; // Currently 17% on my system
    let memory_info = device.memory_info()?; // Currently 1.63/6.37 GB used on my system
    let power_limit = device.enforced_power_limit()?; // 275k milliwatts on my system
    // let power_draw = device.power_usage()?; // Not Implemented yet
    let mem_total_gb = memory_info.total as f64 / 1024.0 / 1024.0 / 1024.0;
    let mem_used_gb = memory_info.used as f64 / 1024.0 / 1024.0 / 1024.0;
    let power_limit_watts = power_limit as f64 / 1000.0;
    // read temperature
    let sensor_gpu: TemperatureSensor = TemperatureSensor::Gpu;
    let temp_board = device.temperature(sensor_gpu)?;  // 42°C on my system

    // clock types
    let clock = Clock::Graphics;
    let clock2 = Clock::Graphics;
    let memory_clock = Clock::Memory;
    let memory_clock2 = Clock::Memory;
    
    // read clock speeds
    let gpu_max_clock = device.max_clock_info(clock)?; // 3105 MHz on my system
    let gpu_current_clock = device.clock_info(clock2)?; // 2505 MHz on my system
    let memory_max_clock = device.max_clock_info(memory_clock)?; // 8501 MHz on my system
    let memory_current_clock = device.clock_info(memory_clock2)?; // 8500 MHz on my system

    let nvda_info = NvdaInfo {
        brand: brand,
        fan_speed_1,
        fan_speed_2,
        memory_used: mem_used_gb,
        memory_total: mem_total_gb,
        power_limit : power_limit_watts,
        temp_board,
        gpu_max_clock,
        gpu_current_clock: gpu_current_clock,
        memory_max_clock,
        mem_current_clock: memory_current_clock,
    };

    return Ok(nvda_info);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvda() {
        let result = nvda();
        assert!(result.is_ok());
    }

    #[test]
    fn test_nvda_output_type() {
        let result = nvda();
        assert!(result.is_ok());
        let nvda_info = result.unwrap();
        assert_eq!(nvda_info.brand, Brand::GeForce); // Adjust this based on your expected brand
        assert!(nvda_info.memory_used >= 0.0);
        assert!(nvda_info.memory_total >= nvda_info.memory_used);
        assert!(nvda_info.power_limit > 0.0);
        assert!(nvda_info.temp_board > 0);
        assert!(nvda_info.gpu_max_clock > 0);
        assert!(nvda_info.gpu_current_clock > 0);
        assert!(nvda_info.memory_max_clock > 0);
        assert!(nvda_info.mem_current_clock > 0);
    }
}


pub fn match_brand (b: Brand) -> &'static str {
    match b {
        Brand::Unknown => "Unknown",
        Brand::Quadro => "Quadro",
        Brand::Tesla => "Tesla",
        Brand::NVS => "NVS",
        Brand::GRID => "GRID",
        Brand::GeForce => "GeForce",
        Brand::Titan => "Titan",
        Brand::VApps => "VApps",
        Brand::VPC => "VPC",
        Brand::VCS => "VCS",
        Brand::VWS => "VWS",
        Brand::CloudGaming => "CloudGaming",
        Brand::VGaming => "VGaming",
        Brand::QuadroRTX => "QuadroRTX",
        Brand::NvidiaRTX => "NvidiaRTX",
        Brand::Nvidia => "Nvidia",
        Brand::GeForceRTX => "GeForceRTX",
        Brand::TitanRTX => "TitanRTX",
    }
}
