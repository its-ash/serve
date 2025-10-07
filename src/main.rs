use actix_files::NamedFile;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::Parser;
use log::{debug, info};
use serde::Serialize;
use std::path::Path;
use std::time::SystemTime;
use tera::{Context, Tera};

// CLI arguments definition
#[derive(Parser, Debug)]
#[clap(
    name = "serve",
    version = "1.0",
    author = "File Server",
    about = "A simple file server for the current directory"
)]
struct Args {
    /// IP address to bind to
    #[clap(short, long, default_value = "127.0.0.1")]
    bind: String,

    /// Port to listen on
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

// Structure for directory entries
#[derive(Debug, Serialize)]
struct DirEntry {
    name: String,
    path: String,
    is_dir: bool,
    size: String,
    last_modified: String,
}

// Structure to hold our application state
struct AppState {
    tera: Tera,
}

// Handler for the root and directory paths
#[get("/{path:.*}")]
async fn serve_dir(
    req: HttpRequest,
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    // Get the requested path and normalize it
    let path_str = path.into_inner();
    let path_str = path_str.replace("%20", " ");

    // Get the base directory (current working directory)
    let base_dir = std::env::current_dir().expect("Failed to get current directory");

    // Construct the full filesystem path
    let req_path = if path_str.is_empty() {
        base_dir.clone()
    } else {
        base_dir.join(&path_str)
    };

    debug!(
        "🔍 Requested path: \x1b[35m{:?}\x1b[0m, Original path: \x1b[36m{}\x1b[0m",
        req_path, path_str
    );

    // If the path is a file, serve it directly
    if req_path.is_file() {
        return Ok(NamedFile::open(req_path)?.into_response(&req));
    }

    // If the path is a directory, serve directory listing
    if req_path.is_dir() {
        let mut entries = Vec::new();
        let mut context = Context::new();
        let req_path_rel = req_path.strip_prefix(&base_dir).unwrap_or(Path::new(""));

        // Add parent directory entry if not in root
        if req_path_rel != Path::new("") {
            // Extract the parent path for navigation
            let parent_path = match req_path_rel.parent() {
                Some(parent) if parent.as_os_str().is_empty() => "/".to_string(),
                Some(parent) => {
                    // Ensure parent path is properly formatted for URLs
                    let parent_str = parent.to_string_lossy().replace("\\", "/");
                    if parent_str.starts_with('/') {
                        parent_str.to_string()
                    } else {
                        format!("/{}", parent_str)
                    }
                }
                None => "/".to_string(),
            };

            entries.push(DirEntry {
                name: "..".to_string(),
                path: parent_path.to_string(),
                is_dir: true,
                size: "-".to_string(),
                last_modified: "-".to_string(),
            });
        }

        // Get all entries in the directory
        for entry in std::fs::read_dir(&req_path)
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to read directory: {}",
                    e
                ))
            })?
            .filter_map(Result::ok)
        {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy().into_owned();
            let metadata = entry.metadata().unwrap_or_else(|_| {
                std::fs::Metadata::from(
                    std::fs::File::open("/dev/null")
                        .unwrap()
                        .metadata()
                        .unwrap(),
                )
            });
            let is_dir = metadata.is_dir();

            // Ensure consistent URL path formatting for all nested levels
            let rel_path = if req_path_rel.as_os_str().is_empty() {
                file_name_str.clone()
            } else {
                // Create a clean path for the URL
                let path_for_url = req_path_rel.to_string_lossy().replace("\\", "/");
                // Make sure the path doesn't start with a slash to avoid double slashes
                let clean_path = path_for_url.trim_start_matches('/');
                format!("{}/{}", clean_path, file_name_str)
            };

            // Format file size
            let size = if is_dir {
                "-".to_string()
            } else {
                format_size(metadata.len())
            };

            // Format last modified time
            let last_modified = metadata
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| {
                    let secs = d.as_secs();
                    let time =
                        chrono::DateTime::<chrono::Utc>::from_timestamp(secs as i64, 0).unwrap();
                    time.format("%Y-%m-%d %H:%M:%S").to_string()
                })
                .unwrap_or_else(|_| "-".to_string());

            entries.push(DirEntry {
                name: file_name_str,
                path: rel_path,
                is_dir,
                size,
                last_modified,
            });
        }

        // Sort directories first, then files (alphabetically)
        entries.sort_by(|a, b| {
            if a.is_dir && !b.is_dir {
                std::cmp::Ordering::Less
            } else if !a.is_dir && b.is_dir {
                std::cmp::Ordering::Greater
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });

        context.insert("entries", &entries);
        // Ensure path is formatted properly for web URLs (replace backslashes with forward slashes)
        let current_path = req_path_rel.to_string_lossy().replace("\\", "/");
        // Remove leading slash to ensure consistent path formatting
        let clean_current_path = current_path.trim_start_matches('/');

        context.insert("current_path", &clean_current_path);
        context.insert("server_name", "File Server");

        // Render the template
        let body = state.tera.render("index.html", &context).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Template error: {}", e))
        })?;

        return Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body));
    }

    // If path doesn't exist
    Err(actix_web::error::ErrorNotFound("Path not found"))
}

// Format file size in human-readable format
fn format_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

// Create HTML template
fn create_template() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "index.html",
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ server_name }}{% if current_path != "" %} - {{ current_path }}{% endif %}</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.3/font/bootstrap-icons.css">
    <style>
        body {
            padding-top: 20px;
            background-color: #f8f9fa;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
        }
        .table {
            background-color: white;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 0 10px rgba(0,0,0,0.05);
        }
        .table th {
            background-color: #f8f9fa;
            border-top: none;
        }
        .dir-link {
            font-weight: bold;
            color: #0d6efd;
        }
        .file-link {
            color: #212529;
        }
        .breadcrumb {
            background-color: white;
            padding: 10px 15px;
            border-radius: 8px;
            box-shadow: 0 0 10px rgba(0,0,0,0.05);
            margin-bottom: 20px;
        }
        .table-responsive {
            border-radius: 8px;
            overflow: hidden;
        }
        .header {
            margin-bottom: 20px;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>{{ server_name }}</h1>
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    <li class="breadcrumb-item"><a href="/">Home</a></li>
                    {% if current_path != "" %}
                        {% set path_parts = current_path | split(pat="/") %}
                        {% set current = "" %}
                        {% for part in path_parts %}
                            {% if loop.last %}
                                <li class="breadcrumb-item active" aria-current="page">{{ part }}</li>
                            {% else %}
                                {% set current = current ~ "/" ~ part %}
                                <li class="breadcrumb-item"><a href="{{ current | safe }}">{{ part }}</a></li>
                            {% endif %}
                        {% endfor %}
                    {% endif %}
                </ol>
            </nav>
        </div>

        <div class="table-responsive">
            <table class="table table-hover">
                <thead>
                    <tr>
                        <th scope="col" style="width: 3%">#</th>
                        <th scope="col" style="width: 47%">Name</th>
                        <th scope="col" style="width: 20%">Size</th>
                        <th scope="col" style="width: 30%">Last Modified</th>
                    </tr>
                </thead>
                <tbody>
                    {% for entry in entries %}
                    <tr>
                        <td>
                            {% if entry.is_dir %}
                            <i class="bi bi-folder-fill text-primary"></i>
                            {% else %}
                            <i class="bi bi-file-text"></i>
                            {% endif %}
                        </td>
                        <td>
                            <a href="/{{ entry.path | safe }}" class="{% if entry.is_dir %}dir-link{% else %}file-link{% endif %}">
                                {{ entry.name }}
                            </a>
                        </td>
                        <td>{{ entry.size }}</td>
                        <td>{{ entry.last_modified }}</td>
                    </tr>
                    {% endfor %}
                </tbody>
            </table>
        </div>

        <footer class="mt-4 text-center text-muted">
            <small>File Server - Running on Rust with Actix Web</small>
        </footer>
    </div>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>"#,
    )
    .expect("Failed to add template");

    tera
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging with custom filter to suppress Actix logs
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter(Some("actix_web"), log::LevelFilter::Error) // Only show errors from actix_web
        .filter(Some("actix_server"), log::LevelFilter::Error) // Only show errors from actix_server
        .filter(Some("actix_http"), log::LevelFilter::Error) // Only show errors from actix_http
        .init();

    // Parse command line arguments
    let args = Args::parse();

    let bind_address = format!("{}:{}", args.bind, args.port);
    let current_dir = std::env::current_dir()?;

    info!("✨ Starting server at http://{}", bind_address);
    info!("📂 Serving directory: {}", current_dir.display());

    // Create app state with Tera template engine
    let app_state = web::Data::new(AppState {
        tera: create_template(),
    });

    // Build and run HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // Don't use the default logger which logs all requests
            // .wrap(middleware::Logger::default())
            .service(serve_dir)
    })
    .bind(&bind_address)?
    .shutdown_timeout(1) // Quick shutdown to avoid verbose logs
    .workers(2) // Reduce number of workers to minimize startup logs
    .run()
    .await
}
