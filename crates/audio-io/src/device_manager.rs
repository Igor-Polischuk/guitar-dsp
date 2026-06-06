use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, SampleFormat};

pub enum DeviceDirection {
    Input,
    Output,
}

#[derive(Debug)]
pub struct AudioIoDevice {
    pub name: String,
    pub is_input: bool,
    pub is_output: bool,
}

pub struct DeviceManager {
    host: Host,
    pub active_input_device: Option<Device>,
    pub active_output_device: Option<Device>,
}

impl DeviceManager {
    pub fn new() -> Self {
        let host = cpal::default_host();
        DeviceManager {
            active_input_device: None,
            active_output_device: None,
            host,
        }
    }
    pub fn get_available_devices(&self) -> Result<Vec<AudioIoDevice>, String> {
        let devices = self.host.devices().map_err(|err| err.to_string())?;
        let mut available_devices = vec![];

        for device in devices {
            if let Ok(device_data) = device.description() {
                available_devices.push(AudioIoDevice {
                    name: device_data.name().to_string(),
                    is_input: device_data.supports_input(),
                    is_output: device_data.supports_output(),
                });
            }
        }

        Ok(available_devices)
    }

    pub fn set_input_device(&mut self, name: &str) -> Result<(), String> {
        let device = self.find_device_by_name(name)?;
        Self::ensure_device_supports(&device, name, DeviceDirection::Input)?;
        self.active_input_device = Some(device);
        Ok(())
    }

    pub fn set_default_input(&mut self) -> Result<(), String> {
        self.active_input_device = Some(
            self.host
                .default_input_device()
                .ok_or("No default input device available".to_string())?,
        );

        Ok(())
    }

    pub fn set_default_output(&mut self) -> Result<(), String> {
        self.active_output_device = Some(
            self.host
                .default_output_device()
                .ok_or("No default output device available".to_string())?,
        );

        Ok(())
    }

    pub fn set_output_device(&mut self, name: &str) -> Result<(), String> {
        let device = self.find_device_by_name(name)?;
        Self::ensure_device_supports(&device, name, DeviceDirection::Output)?;
        self.active_output_device = Some(device);
        Ok(())
    }

    fn find_device_by_name(&self, name: &str) -> Result<Device, String> {
        let devices = self.host.devices().map_err(|err| err.to_string())?;

        for device in devices {
            if let Ok(device_data) = device.description() {
                if device_data.name() == name {
                    return Ok(device);
                }
            }
        }

        Err(format!("Device '{name}' not found"))
    }

    fn ensure_device_supports(
        device: &Device,
        device_name: &str,
        direction: DeviceDirection,
    ) -> Result<(), String> {
        let is_supported = match direction {
            DeviceDirection::Input => device
                .supported_input_configs()
                .map(|mut configs| configs.next().is_some()),
            DeviceDirection::Output => device
                .supported_output_configs()
                .map(|mut configs| configs.next().is_some()),
        }
        .map_err(|err| format!("Failed to read configs for device '{device_name}': {err}"))?;

        if is_supported {
            Ok(())
        } else {
            let direction = match direction {
                DeviceDirection::Input => "input",
                DeviceDirection::Output => "output",
            };
            Err(format!(
                "Device '{device_name}' does not support audio {direction}"
            ))
        }
    }
}
