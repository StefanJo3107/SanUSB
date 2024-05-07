use embedded_svc::ipv4::IpInfo;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;

pub struct WiFiHandler {
    ssid: String,
    password: String,
    wifi: BlockingWifi<EspWifi<'static>>,
}

impl WiFiHandler {
    pub fn new(ssid: String, password: String) -> anyhow::Result<WiFiHandler> {
        let peripherals = Peripherals::take()?;
        let sys_loop = EspSystemEventLoop::take()?;
        let nvs = EspDefaultNvsPartition::take()?;
        let wifi = BlockingWifi::wrap(
            EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
            sys_loop,
        )?;

        Ok(WiFiHandler {
            ssid,
            password,
            wifi,
        })
    }

    pub fn connect_wifi(&mut self) -> anyhow::Result<()> {
        let configuration = Configuration::Client(
            ClientConfiguration {
                ssid: self.ssid.clone().try_into().unwrap(),
                bssid: None,
                auth_method: AuthMethod::WPA2Personal,
                password: self.password.clone().try_into().unwrap(),
                channel: None,
                ..Default::default()
            }
        );

        self.wifi.set_configuration(&configuration)?;

        self.wifi.start()?;
        info!("WiFi started");
        self.wifi.connect()?;
        info!("WiFi connected");
        self.wifi.wait_netif_up()?;
        info!("WiFi netif up");

        Ok(())
    }

    pub fn get_ip_info(&self) -> anyhow::Result<IpInfo> {
        Ok(self.wifi.wifi().sta_netif().get_ip_info()?)
    }
}