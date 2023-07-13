//! # Rocket Client Library
//!
//! This library provides a Rocket client with metric collection for endpoints. It allows you to create and configure a Rocket server instance to interact with web services.
//!
//! ## Example Usage
//!
//! ```rust
//! use valensas_actuator::metrics::{ArcRwLockPrometheus, PrometheusMetrics};
//! use valensas_rocket::client::client_params::ClientParams;
//! use valensas_rocket::client::client_service::Client;
//! use rocket::{get, routes, Route};
//! use std::sync::{Arc, RwLock};
//!
//! #[get("/")]
//! fn index() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a vector of routes (endpoints)
//!     let routes: Vec<Route> = routes![index];
//!
//!     // Optional: Create a Prometheus metrics object
//!     // For further information about actuator library, visit: https://crates.io/crates/valensas-actuator
//!     let prometheus = Arc::new(RwLock::new(PrometheusMetrics::new("your_namespace")));
//!     let prometheus_fairing = ArcRwLockPrometheus::new(prometheus.clone());
//!
//!     // Create a new Rocket client
//!     let client = Client::new(
//!         ClientParams {
//!             ip_addr: "127.0.0.1".to_string(),
//!             port: "8000".to_string(),
//!         },
//!         routes,
//!         Some(prometheus_fairing), // Pass the Prometheus object
//!     )
//!         // Optional: Set a managed state
//!         //.set_manage(SomeState)
//!         // Optional: Set a fairing
//!         //.set_fairing(SomeFairing)
//!         .spawn_rocket();
//!
//!     rocket::tokio::task::spawn(client.await)
//!         .await
//!         .unwrap()
//!         .unwrap();
//!     println!("Rocket server launched successfully!");
//! }
//! ```
//!
//! The example demonstrates how to create a Rocket client using the library. It sets up a single route ("/") with a handler function (`index`). Optionally, it creates a Prometheus metrics object, sets a managed state (`SomeState`), and attaches a fairing (`SomeFairing`). The client is then launched using the `spawn_rocket` function, and the Prometheus metrics can be accessed if available (For further information about metrics, visit: [valensas-actuator](https://crates.io/crates/valensas-actuator)). Finally, the client awaits completion and prints a success message.

pub mod client;
