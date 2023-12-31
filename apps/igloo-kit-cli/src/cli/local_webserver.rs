use super::display::Message;
use super::display::MessageType;

use crate::cli::routines::stop::StopLocalInfrastructure;
use crate::cli::routines::Routine;
use crate::cli::routines::RunMode;
use crate::framework::controller::RouteMeta;
use crate::infrastructure::olap;

use crate::infrastructure::olap::clickhouse::ConfiguredDBClient;
use crate::infrastructure::stream::redpanda;
use crate::infrastructure::stream::redpanda::ConfiguredProducer;

use crate::project::Project;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper::Request;
use hyper::Response;
use hyper::StatusCode;
use hyper_util::rt::TokioIo;
use hyper_util::{rt::TokioExecutor, server::conn::auto};
use log::debug;
use log::error;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
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

struct RouteService {
    route_table: Arc<Mutex<HashMap<PathBuf, RouteMeta>>>,
    configured_producer: Arc<Mutex<ConfiguredProducer>>,
    configured_db_client: Arc<Mutex<ConfiguredDBClient>>,
}

impl Service<Request<Incoming>> for RouteService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        Box::pin(router(
            req,
            self.route_table.clone(),
            self.configured_producer.clone(),
            self.configured_db_client.clone(),
        ))
    }
}

fn options_route() -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type, Baggage, Sentry-Trace",
        )
        .body(Full::new(Bytes::from("Success")))
        .unwrap();

    Ok(response)
}

async fn ingest_route(
    req: Request<hyper::body::Incoming>,
    route: PathBuf,
    configured_producer: Arc<Mutex<ConfiguredProducer>>,
    route_table: Arc<Mutex<HashMap<PathBuf, RouteMeta>>>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    show_message!(
        MessageType::Info,
        Message {
            action: "POST".to_string(),
            details: route.to_str().unwrap().to_string().to_string(),
        }
    );
    if route_table.lock().await.contains_key(&route) {
        let body = req.collect().await.unwrap().to_bytes().to_vec();

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
                show_message!(
                    MessageType::Success,
                    Message {
                        action: "SUCCESS".to_string(),
                        details: route.to_str().unwrap().to_string(),
                    }
                );
                Ok(Response::new(Full::new(Bytes::from("SUCCESS"))))
            }
            Err(e) => {
                println!("Error: {:?}", e);
                Ok(Response::new(Full::new(Bytes::from("Error"))))
            }
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from(
                "Please visit /console to view your routes",
            )))
    }
}

async fn console_route(
    configured_db_client: Arc<Mutex<ConfiguredDBClient>>,
    configured_producer: Arc<Mutex<ConfiguredProducer>>,
    route_table: Arc<Mutex<HashMap<PathBuf, RouteMeta>>>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    show_message!(
        MessageType::Info,
        Message {
            action: "GET".to_string(),
            details: "Console API".to_string(),
        }
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

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type, Baggage, Sentry-Trace",
        )
        .body(Full::new(Bytes::from(
            json!({
                "tables": tables,
                "topics": topics,
                "routes": routes_table
            })
            .to_string(),
        )))?;
    Ok(response)
}

async fn router(
    req: Request<hyper::body::Incoming>,
    route_table: Arc<Mutex<HashMap<PathBuf, RouteMeta>>>,
    configured_producer: Arc<Mutex<ConfiguredProducer>>,
    configured_db_client: Arc<Mutex<ConfiguredDBClient>>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
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

    let route_split = route.to_str().unwrap().split('/').collect::<Vec<&str>>();

    match (req.method(), &route_split[..]) {
        (&hyper::Method::POST, ["ingest", _]) => {
            ingest_route(req, route, configured_producer, route_table).await
        }

        (&hyper::Method::GET, ["console"]) => {
            console_route(configured_db_client, configured_producer, route_table).await
        }
        (&hyper::Method::GET, ["console", "routes"]) => {
            todo!("get all routes");
        }
        (&hyper::Method::GET, ["console", "routes", _route_id]) => {
            todo!("get specific route");
        }

        (&hyper::Method::GET, ["console", "tables"]) => {
            todo!("get all tables");
        }
        (&hyper::Method::GET, ["console", "tables", _table_name]) => {
            todo!("get specific table");
        }

        (&hyper::Method::OPTIONS, _) => options_route(),
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("no match"))),
    }
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
        route_table: Arc<Mutex<HashMap<PathBuf, RouteMeta>>>,
        project: &Project,
    ) {
        //! Starts the local webserver
        let socket = self.socket().await;

        // We create a TcpListener and bind it to 127.0.0.1:3000
        let listener = TcpListener::bind(socket).await.unwrap();

        let producer = Arc::new(Mutex::new(redpanda::create_producer(
            project.redpanda_config.clone(),
        )));
        let db_client = Arc::new(Mutex::new(olap::clickhouse::create_client(
            project.clickhouse_config.clone(),
        )));

        show_message!(
            MessageType::Info,
            Message {
                action: "Started".to_string(),
                details: format!(" server on port {}", socket.port()),
            }
        );

        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
        let mut sigint =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()).unwrap();

        loop {
            tokio::select! {
                _ = sigint.recv() => {
                    let run_mode = RunMode::Explicit;
                    StopLocalInfrastructure::new(run_mode).run(run_mode).unwrap();
                    std::process::exit(0);
                }
                _ = sigterm.recv() => {
                    let run_mode = RunMode::Explicit;
                    StopLocalInfrastructure::new(run_mode).run(run_mode).unwrap();
                    std::process::exit(0);
                }
                listener_result = listener.accept() => {
                    let (stream, _) = listener_result.unwrap();
                    // Use an adapter to access something implementing `tokio::io` traits as if they implement
                    // `hyper::rt` IO traits.
                    let io = TokioIo::new(stream);

                    let route_table = route_table.clone();
                    let producer = producer.clone();
                    let db_client = db_client.clone();

                    // Spawn a tokio task to serve multiple connections concurrently
                    tokio::task::spawn(async move {
                        // Run this server for... forever!
                        if let Err(e) = auto::Builder::new(TokioExecutor::new()).serve_connection(
                                io,
                                RouteService {
                                    route_table,
                                    configured_producer: producer,
                                    configured_db_client: db_client,
                                },
                            ).await {
                                error!("server error: {}", e);
                            }

                    });
                }
            }
        }
    }
}
