use eureka_client::{BaseConfig, EurekaClient, PortData};

pub fn init_eureka(
) -> EurekaClient {
    let mut config = BaseConfig::default();
    config.instance.ip_addr = "localhost".to_string();
    config.instance.port = Some(PortData::new(8082, true));
    let eureka = EurekaClient::new(config);
    eureka.start();
    eureka
}
