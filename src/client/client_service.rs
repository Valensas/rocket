#![allow(non_snake_case)]

use std::{
    future::Future,
    net::{IpAddr, Ipv4Addr},
};

use mpc_prometheus::metrics::metrics::ArcRwLockPrometheus;
use rocket::{fairing::Fairing, Build, Error, Ignite, Rocket, Route};

use super::client_params::ClientParams;

pub struct Client {
    params: ClientParams,
    pub rocket_server: Rocket<Build>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        let custom = rocket::Config {
            address: IpAddr::V4(self.params.ip_addr.parse::<Ipv4Addr>().unwrap()),
            port: self.params.port.parse::<u16>().unwrap(),
            ..Default::default()
        };
        Self {
            rocket_server: rocket::custom(custom),
            params: self.params.clone(),
        }
    }
}

impl Client {
    pub fn new(
        params: ClientParams,
        endpoints: Vec<Route>,
        prometheus: Option<ArcRwLockPrometheus>,
    ) -> Self {
        let mut custom = rocket::config::Config::release_default();
        let ip_address = params.ip_addr.parse::<Ipv4Addr>();
        let port = params.port.parse::<u16>();
        if let Ok(ip) = ip_address {
            custom.address = IpAddr::V4(ip);
        }
        if let Ok(prt) = port {
            custom.port = prt;
        }

        let rocket_server = match prometheus {
            Some(prometheus) => rocket::custom(custom)
                .attach(prometheus.clone())
                .manage(prometheus)
                .mount("/", endpoints),
            None => rocket::custom(custom).mount("/", endpoints),
        };

        Self {
            rocket_server,
            params,
        }
    }

    pub fn set_manage<T>(mut self, manage: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.rocket_server = self.rocket_server.manage(manage);
        self
    }

    pub fn set_fairing<T>(mut self, attachment: T) -> Self
    where
        T: Fairing,
    {
        self.rocket_server = self.rocket_server.attach(attachment);
        self
    }

    pub async fn spawn_rocket(self) -> impl Future<Output = Result<Rocket<Ignite>, Error>> {
        let rocket = match self.rocket_server.ignite().await {
            Ok(res) => res,
            Err(err) => {
                panic!("{:#?}", err);
            }
        };
        rocket.launch()
    }
}
