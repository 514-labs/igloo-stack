use super::display::show_message;
use super::display::Message;
use super::display::MessageType;

use super::CommandTerminal;
use crate::framework::controller::RouteMeta;
use crate::infrastructure::olap;

use crate::infrastructure::olap::clickhouse::ConfiguredDBClient;
use crate::infrastructure::stream::redpanda;
use crate::infrastructure::stream::redpanda::ConfiguredProducer;

use crate::project::Project;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;
use hyper::StatusCode;
use log::debug;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalWebserverConfig {
    pub host: String,
    pub port: u16,
}

impl LocalWebserverConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

impl Default for LocalWebserverConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 4000,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RouteInfo {
    pub route_path: String,
    pub file_path: String,
    pub table_name: String,
    pub view_name: Option<String>,
}

impl RouteInfo {
    pub fn new(
        route_path: String,
        file_path: String,
        table_name: String,
        view_name: Option<String>,
    ) -> Self {
        Self {
            route_path,
            file_path,
            table_name,
            view_name,
        }
    }
}

async fn handler(
    req: Request<Body>,
    term: Arc<RwLock<CommandTerminal>>,
    route_table: Arc<Mutex<HashMap<PathBuf, RouteMeta>>>,
    configured_producer: Arc<Mutex<ConfiguredProducer>>,
    configured_db_client: Arc<Mutex<ConfiguredDBClient>>,
) -> Result<Response<String>, hyper::http::Error> {
    debug!(
        "HTTP Request Received: {:?}, with Route Table {:?}",
        req, route_table
    );

    let route_prefix = PathBuf::from("/");
    let route = PathBuf::from(req.uri().path())
        .strip_prefix(route_prefix)
        .unwrap()
        .to_path_buf()
        .clone();

    debug!(
        "Processing route: {:?}, with Route Table {:?}",
        route, route_table
    );

    // Check if route is in the route table
    if route_table.lock().await.contains_key(&route) || route == PathBuf::from("console") {
        match req.method() {
            &hyper::Method::POST => {
                show_message(
                    term.clone(),
                    MessageType::Info,
                    Message {
                        action: "POST".to_string(),
                        details: route.to_str().unwrap().to_string().to_string(),
                    },
                );

                let bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
                let body = String::from_utf8(bytes.to_vec()).unwrap();

                let guard = route_table.lock().await;
                let topic_name = &guard.get(&route).unwrap().table_name;

                let res = configured_producer
                    .lock()
                    .await
                    .producer
                    .send(
                        FutureRecord::to(topic_name)
                            .key(topic_name) // This should probably be generated by the client that pushes data to the API
                            .payload(&body),
                        Timeout::After(Duration::from_secs(1)),
                    )
                    .await;

                match res {
                    Ok(_) => {
                        show_message(
                            term.clone(),
                            MessageType::Success,
                            Message {
                                action: "SUCCESS".to_string(),
                                details: route.to_str().unwrap().to_string(),
                            },
                        );
                        return Ok(Response::new("SUCCESS".to_string()));
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return Ok(Response::new("ERROR".to_string()));
                    }
                }
            }
            &hyper::Method::OPTIONS => {
                show_message(
                    term.clone(),
                    MessageType::Info,
                    Message {
                        action: "OPTIONS".to_string(),
                        details: route.to_str().unwrap().to_string(),
                    },
                );
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "POST, OPTIONS")
                    .header(
                        "Access-Control-Allow-Headers",
                        "Content-Type, Baggage, Sentry-Trace",
                    )
                    .body("".to_string())
                    .unwrap();

                return Ok(response);
            }
            // Get method allows the user to navigate to /console route only
            &hyper::Method::GET => {
                show_message(
                    term.clone(),
                    MessageType::Info,
                    Message {
                        action: "GET".to_string(),
                        details: route.to_str().unwrap().to_string(),
                    },
                );

                let db_guard = configured_db_client.lock().await;
                let producer_guard = configured_producer.lock().await;
                let route_table_guard = route_table.lock().await;

                let tables = olap::clickhouse::fetch_all_tables(&db_guard).await.unwrap();
                let topics = redpanda::fetch_topics(&producer_guard.config)
                    .await
                    .unwrap();
                let routes_table: Vec<RouteInfo> = route_table_guard
                    .clone()
                    .iter()
                    .map(|(k, v)| {
                        RouteInfo::new(
                            k.to_str().unwrap().to_string(),
                            v.original_file_path.to_str().unwrap().to_string(),
                            v.table_name.clone(),
                            v.view_name.clone(),
                        )
                    })
                    .collect();

                if route == PathBuf::from("console") {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Access-Control-Allow-Methods", "GET")
                        .header(
                            "Access-Control-Allow-Headers",
                            "Content-Type, Baggage, Sentry-Trace",
                        )
                        .body(
                            json!({
                                "tables": tables,
                                "topics": topics,
                                "routes": routes_table
                            })
                            .to_string(),
                        )?;
                    return Ok(response);
                }
            }
            _ => {
                show_message(
                    term.clone(),
                    MessageType::Info,
                    Message {
                        action: "UNKNOWN METHOD".to_string(),
                        details: route.to_str().unwrap().to_string(),
                    },
                );
                // If not, return a 404
                return Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(
                        "Please use a POST method to send data to your ingestion point".to_string(),
                    );
            }
        }
    }

    // If not, return a 404
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("NOTFOUND".to_string())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Webserver {
    host: String,
    port: u16,
}

impl Webserver {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub async fn socket(&self) -> SocketAddr {
        tokio::net::lookup_host(format!("{}:{}", self.host, self.port))
            .await
            .unwrap()
            .next()
            .unwrap()
    }

    pub async fn start(
        &self,
        term: Arc<RwLock<CommandTerminal>>,
        route_table: Arc<Mutex<HashMap<PathBuf, RouteMeta>>>,
        project: &Project,
    ) {
        let socket = self.socket().await;

        show_message(
            term.clone(),
            MessageType::Info,
            Message {
                action: "starting".to_string(),
                details: format!(" server on port {}", socket.port()),
            },
        );

        let producer = Arc::new(Mutex::new(redpanda::create_producer(
            project.redpanda_config.clone(),
        )));
        let db_client = Arc::new(Mutex::new(olap::clickhouse::create_client(
            project.clickhouse_config.clone(),
        )));

        let main_service = make_service_fn(move |_| {
            let route_table = route_table.clone();
            let producer = producer.clone();
            let db_client = db_client.clone();
            let term = term.clone();

            async {
                Ok::<_, Infallible>(service_fn(move |req| {
                    handler(
                        req,
                        term.clone(),
                        route_table.clone(),
                        producer.clone(),
                        db_client.clone(),
                    )
                }))
            }
        });

        let server = Server::bind(&socket).serve(main_service);

        // Run this server for... forever!
        if let Err(e) = server.await {
            println!("server error: {}", e)
        }
    }
}
