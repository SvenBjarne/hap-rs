// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, temperature_sensor, HapService},
    Result,
};

/// Temperature Sensor Accessory.
pub type TemperatureSensor = Accessory<TemperatureSensorInner>;

/// Inner type of the Temperature Sensor Accessory.
#[derive(Default)]
pub struct TemperatureSensorInner {
    /// ID of the Temperature Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Temperature Sensor Service.
    pub temperature_sensor: temperature_sensor::TemperatureSensor,
}

impl HapAccessory for TemperatureSensorInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![&self.accessory_information, &self.temperature_sensor]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.temperature_sensor]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation { &mut self.accessory_information }

    fn init_iids(&mut self, accessory_id: u64, event_emitter: EventEmitterPtr) -> Result<()> {
        let mut next_iid = 1;
        for service in self.get_mut_services() {
            service.set_id(next_iid);
            next_iid += 1;
            for characteristic in service.get_mut_characteristics() {
                characteristic.set_id(next_iid)?;
                characteristic.set_accessory_id(accessory_id)?;
                characteristic.set_event_emitter(Some(event_emitter.clone()))?;
                next_iid += 1;
            }
        }
        Ok(())
    }
}

/// Creates a new Temperature Sensor Accessory.
pub fn new(information: Information) -> Result<TemperatureSensor> {
    let mut temperature_sensor = temperature_sensor::new();
    temperature_sensor.set_primary(true);
    Ok(TemperatureSensor::new(TemperatureSensorInner {
        accessory_information: information.to_service()?,
        temperature_sensor,
        ..Default::default()
    }))
}