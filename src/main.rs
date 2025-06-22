use axum::{
    extract::Json,
    routing::post,
    Router,
};
use tracing::{info, Level};
use tracing_subscriber::fmt::format::FmtSpan;
use prometheus::{Encoder, TextEncoder, Registry, IntCounter};
use std::sync::Arc;
use axum::response::IntoResponse;
use axum_server::tls_rustls::RustlsConfig;
use k8s_openapi::api::core::v1::Pod;
use kube::core::admission::AdmissionReview;
use kube::core::admission::AdmissionResponse;
use k8s_openapi::api::core::v1::Container;

struct AppState {
    registry: Registry,
    requests_total: IntCounter,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_span_events(FmtSpan::CLOSE)
        .with_max_level(Level::INFO)
        .init();

    let registry = Registry::new();
    let requests_total = IntCounter::new("requests_total", "Total requests").unwrap();
    registry.register(Box::new(requests_total.clone())).unwrap();

    let state = Arc::new(AppState { registry, requests_total });

    let app = Router::new()
        .route("/admission", post(admission_handler))
        .route("/metrics", post(metrics_handler).get(metrics_handler))
        .with_state(state.clone());

    let config = RustlsConfig::from_pem_file(
        "/tls/tls.crt",
        "/tls/tls.key",
    ).await.unwrap();

    let port = get_port();
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .unwrap();
}

fn get_port() -> u16 {
    std::env::var("WEBHOOK_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8443)
}

async fn admission_handler(
    state: axum::extract::State<Arc<AppState>>,
    Json(admission_review): Json<AdmissionReview<Pod>>,
) -> impl IntoResponse {
    state.requests_total.inc();

    // Логирование образов контейнеров, если есть request/object/spec/containers
    if let Some(request) = &admission_review.request {
        if let Some(pod) = &request.object {
            if let Some(spec) = pod.spec.as_ref() {
                log_containers(&spec.containers, "container");
                if let Some(init_containers) = &spec.init_containers {
                    log_containers(&init_containers, "init container");
                }
            }
        }
        let admission_review_response = AdmissionReview {
            types: admission_review.types,
            request: None,
            response: Some(AdmissionResponse::from(request)),
        };
        return Json(admission_review_response);
    }

    Json(AdmissionReview::<Pod> {
        types: admission_review.types,
        request: None,
        response: None,
    })
}

fn log_containers(containers: &Vec<Container>, kind: &str) {
    for c in containers {
        if let Some(image) = &c.image {
            let (registry, name, tag, digest) = parse_image(&image);
            info!(kind, registry, name, tag, digest, "container image info");
        }
    }
}

fn parse_image(image: &str) -> (String, String, String, String) {
    // Примитивный парсер docker-образа: [registry/][repo/]name[:tag][@digest]
    let mut registry = String::new();
    let name: String;
    let mut tag = String::from("latest");
    let mut digest = String::new();

    let mut img = image;
    if let Some(at) = img.find('@') {
        digest = img[at + 1..].to_string();
        img = &img[..at];
    }
    if let Some(colon) = img.rfind(':') {
        tag = img[colon + 1..].to_string();
        img = &img[..colon];
    }
    let parts: Vec<&str> = img.split('/').collect();
    if parts.len() > 1 && (parts[0].contains('.') || parts[0].contains(':')) {
        registry = parts[0].to_string();
        name = parts[1..].join("/");
    } else {
        name = img.to_string();
    }
    (registry, name, tag, digest)
}

async fn metrics_handler(
    state: axum::extract::State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let mf = state.registry.gather();
    encoder.encode(&mf, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
