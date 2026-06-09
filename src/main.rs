// Sample Rust actix-web app for the Contixo sample-repo collection.
//
// Rust isn't natively supported by paketo-buildpacks (it requires
// cargo-buildpack which isn't shipped in the default builder), so
// this sample uses the kaniko/Dockerfile path. Demonstrates State A
// of the wizard: Dockerfile has EXPOSE 8000, so the parser fills in
// the port automatically and the customer hits Continue without
// typing anything.
//
// Listens on $PORT (Dockerfile's EXPOSE 8000 is what the parser
// uses; the customer can override at deploy time but defaults to
// 8000).

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn home() -> impl Responder {
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());

    let html = format!(r#"<!doctype html>
<html><head><title>Contixo Sample · Rust + Dockerfile</title>
<style>
  body {{ font-family: -apple-system, BlinkMacSystemFont, sans-serif; max-width: 720px; margin: 4rem auto; padding: 0 1rem; color: #1e293b; }}
  h1 {{ font-size: 2rem; margin-bottom: 0.25rem; }}
  .badge {{ display: inline-block; background: #fce7f3; color: #9d174d; padding: 0.25rem 0.75rem; border-radius: 999px; font-size: 0.75rem; font-weight: 600; }}
  .meta {{ margin-top: 2rem; background: #f8fafc; padding: 1rem; border-radius: 0.5rem; font-family: monospace; font-size: 0.85rem; }}
</style></head>
<body>
  <span class="badge">✓ Built with kaniko (Dockerfile path · State A)</span>
  <h1>Hello from Contixo!</h1>
  <p>This is a Rust actix-web app deployed via the Contixo dynamic-site wizard.</p>
  <p>The repo had a <code>Dockerfile</code> with <code>EXPOSE 8000</code> → the wizard parsed it (State A — all configuration detected) → kaniko built the multi-stage image → final stage's static binary runs in the pod.</p>
  <div class="meta">
    <strong>Runtime:</strong> Rust + actix-web<br>
    <strong>Port:</strong> {port}<br>
    <strong>Hostname:</strong> {hostname}<br>
    <strong>Path:</strong> /
  </div>
</body></html>"#);

    HttpResponse::Ok().content_type("text/html").body(html)
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "runtime": "rust",
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    println!("contixo sample (rust-actix) listening on :{}", port);
    HttpServer::new(|| App::new().service(home).service(healthz))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}
