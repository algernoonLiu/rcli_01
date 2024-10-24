use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use axum::{extract::{Path, State}, http::StatusCode, routing::get, Router};
use anyhow::Result;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port {}", path.clone(), addr);
    let state = HttpServeState { path: path.clone() };
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>, 
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, format!("File {} not found!", p.display()))
    } else {
        if p.is_dir() {
            let out_path = p.clone().join("index.html");
            // 如果是目录，返回目录内容列表 
            // 目录下所有文件输出到新建的index.html文件中， 格式 <html><body><ul>...</ul></body></html>
            // 使用 tokio 读取目录下所有文件
            let mut files: Vec<String> = Vec::new();
            let mut entries = tokio::fs::read_dir(p).await.unwrap();

            while let Some(entry) = entries.next_entry().await.unwrap() {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                files.push(file_name.to_string());
                
            }
            // 按照文件名排序， 每一条格式 <li><a href="path">path</a></li>
            files.sort();
            let mut html = String::new();
            html.push_str("<html><body><ul>");
            for file in files {
                html.push_str(&format!("<li><a href=\"{}\">{}</a></li>", file, file));
            }
            html.push_str("</ul></body></html>");
            // html 中的内容 渲染返回
            tokio::fs::write(out_path, html).await.unwrap();
            (StatusCode::OK, "None".to_string())
        } else {
            // 如果文件存在，读取文件内容并返回
            match tokio::fs::read_to_string(p).await {
                Ok(content) => {
                    info!("Read {} bytes", content.len());
                    (StatusCode::OK, content)
                },
                Err(err) => {
                    warn!("Error reading file {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                },
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = HttpServeState { path: PathBuf::from(".") };
        let (status, content) = file_handler(State(Arc::new(state)), Path("fixtures".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.contains("[package]"));

    }
}
