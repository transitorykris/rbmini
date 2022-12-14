use btleplug::api::{Central, Manager, Peripheral, ScanFilter};
use std::time::Duration;
use tokio::time;
use uuid::{uuid, Uuid};

const RACEBOX_LOCAL_NAME: &str = "RaceBox Mini";

// RaceBox mini characteristics
const DEVICE_INFO_CHAR: Uuid = uuid!("0000180a-0000-1000-8000-00805f9b34fb");
const MODEL_CHAR: Uuid = uuid!("00002a24-0000-1000-8000-00805f9b34fb");
const SERIAL_NUMBER_CHAR: Uuid = uuid!("00002a25-0000-1000-8000-00805f9b34fb");
const FIRMWARE_REV_CHAR: Uuid = uuid!("00002a26-0000-1000-8000-00805f9b34fb");
const HARDWARE_REV_CHAR: Uuid = uuid!("00002a27-0000-1000-8000-00805f9b34fb");
const MANUFACTURER_CHAR: Uuid = uuid!("00002a29-0000-1000-8000-00805f9b34fb");
const UART_SERVICE_CHAR: Uuid = uuid!("6E400001-B5A3-F393-E0A9-E50E24DCCA9E");
const RX_CHAR: Uuid = uuid!("6E400002-B5A3-F393-E0A9-E50E24DCCA9E");
const TX_CHAR: Uuid = uuid!("6E400003-B5A3-F393-E0A9-E50E24DCCA9E");

// btle connection management
pub struct RbConnection {
    adapter_list: Vec<btleplug::platform::Adapter>,
    manager: Box<btleplug::platform::Manager>,
    peripherals: Vec<btleplug::platform::Peripheral>,
}

impl RbConnection {
    pub async fn new() -> Result<RbConnection, String> {
        let manager = Box::new(btleplug::platform::Manager::new().await.unwrap());

        let adapter_list = manager.adapters().await.unwrap();
        if adapter_list.is_empty() {
            return Err(String::from("No adapters found"));
        }
        let mut peripherals = Vec::new();
        for adapter in adapter_list.iter() {
            adapter.start_scan(ScanFilter::default()).await;
            time::sleep(Duration::from_secs(10)).await;
            peripherals = adapter.peripherals().await.unwrap();
            if peripherals.is_empty() {
                return Err(String::from("No devices found"));
            }
        }

        Ok(RbConnection {
            adapter_list,
            manager,
            peripherals,
        })
    }
}
