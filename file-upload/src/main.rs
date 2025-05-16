use std::sync::Arc;

use dioxus::{html::FileEngine, prelude::*};
use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        UploadFile {}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct File {
    pub filename: String,
    pub data: Vec<u8>,
}

#[component]
pub fn UploadFile() -> Element {
    let mut upload_file_name = use_signal(|| String::new());
    let read_files = move |fe: Arc<dyn FileEngine>| async move {
        let files = fe.files();
        if let Some(filename) = files.iter().next() {
            let buf = fe.read_file(&filename).await.unwrap();
            let res = upload_file_server(File {
                filename: filename.clone(),
                data: buf,
            })
            .await;
            upload_file_name.set(res.unwrap());
        }
    };
    let upload_file = move |e: FormEvent| async move {
        if let Some(fe) = e.files() {
            read_files(fe).await;
        }
    };
    rsx! {
        div {
            input { r#type: "file", multiple: false, onchange: upload_file }
        }
        if !upload_file_name().is_empty() {
            div { "上传成功：{upload_file_name}" }
        }
    }
}

#[server]
async fn upload_file_server(file: File) -> Result<String, ServerFnError> {
    // 获取扩展名
    let ext_name = file.filename.split('.').last().unwrap();
    // 生成文件名
    let filename = format!("{}.{}", xid::new().to_string(), ext_name);
    // 写入文件
    tokio::fs::write(&filename, file.data).await?;
    Ok(filename)
}
