use dioxus::{html::FileEngine, prelude::*};
use std::sync::Arc;

use crate::components::dialog::Alert;
use crate::components::{ImageUploadIcon, LoadIcon};

const MAX_FILE_SIZE: usize = 2 * 1024 * 1024;
const ACCEPT_FILE_TYPE: &str = "image/*";
const ACCEPT_FILE_PREFIX: &str = "image/";

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct File {
    pub filename: String,
    pub data: Vec<u8>,
    pub content_type: String,
}
#[cfg(feature = "server")]
impl File {
    fn ext_name(&self) -> String {
        self.filename
            .split(".")
            .last()
            .unwrap_or_default()
            .to_string()
    }

    fn is_allow(&self) -> bool {
        self.content_type.starts_with(ACCEPT_FILE_PREFIX)
    }

    fn is_size(&self) -> bool {
        self.data.len() < MAX_FILE_SIZE
    }
}

#[component]
pub fn Upload() -> Element {
    let mut uploaded_file_list = use_signal(Vec::new);
    let mut err_msg = use_signal(|| String::new());
    let mut uploading = use_signal(|| false);

    let read_files = move |fe: Arc<dyn FileEngine>| async move {
        uploading.set(true);
        let files = fe.files();
        let mut readed_files = vec![];
        for filename in &files {
            let content_type = mime_guess::from_path(filename)
                .first_or_octet_stream()
                .to_string();
            if !content_type.starts_with(ACCEPT_FILE_PREFIX) {
                err_msg.set("不支持的文件格式".into());
                break;
            }
            let buf = fe.read_file(filename).await.unwrap();

            if buf.len() > MAX_FILE_SIZE {
                err_msg.set("文件过大".into());
                break;
            }

            readed_files.push(File {
                filename: filename.to_string(),
                data: buf,
                content_type,
            });
        }

        let total_file_size: usize = readed_files.iter().map(|f| f.data.len()).sum();

        if total_file_size > MAX_FILE_SIZE {
            err_msg.set("文件过大".into());
            uploading.set(false);
            return;
        }
        if !readed_files.is_empty() {
            let mut file_list = match upload_server(readed_files).await {
                Ok(v) => v,
                Err(e) => {
                    err_msg.set(e.to_string());
                    uploading.set(false);
                    return;
                }
            };

            uploaded_file_list.write().append(&mut file_list);
        }
        uploading.set(false);
    };
    let hanlde_upload_files = move |e: FormEvent| async move {
        if let Some(fe) = e.files() {
            read_files(fe).await;
        }
    };
    rsx! {
        div { class: "space-y-6",
            div { class: "bg-white/70 h-64 w-full  relative",
                label {
                    r#for: "upload",
                    class: "cursor-pointer block w-full h-full flex justify-center items-center  border border-inset border-dashed border-gray-400 hover:border-purple-400 hover:text-purple-600",
                    div { class: "flex justify-center items-center flex-col gap-y-3",
                        ImageUploadIcon { size: 32 }
                        "上传图片"
                    }
                }
                input {
                    id: "upload",
                    r#type: "file",
                    accept: "{ACCEPT_FILE_TYPE}",
                    class: "hidden",
                    onchange: hanlde_upload_files,
                }
                if uploading() {
                    div { class: "absolute top-0 left-0 w-full h-full flex justify-center items-center bg-white z-[1]  border border-inset border-dashed border-gray-400",
                        div { class: "flex justify-center items-center flex-col gap-y-3",
                            LoadIcon {
                                size: 32,
                                class: "animate-spin text-purple-700",
                            }
                            "正在上传"
                        }
                    }
                }
            }
            ul { class: "grid grid-cols-2 gap-4 lg:grid-cols-5",
                for i in uploaded_file_list() {
                    li {
                        key: "{i}",
                        class: "h-48 w-full lg:h-48 overflow-hidden border border-gray-400 focus-within:border-purple-400 rounded-md shadow-sm",
                        img { src: "{i}", class: "h-40 w-full object-cover" }
                        div { class: "h-8 w-full",
                            input {
                                class: "block w-full p-1 outline-none text-sm text-gray-600 focus:text-inherit",
                                value: "{i}",
                            }
                        }
                    }
                }

            }
        }

        if !err_msg().is_empty() {
            Alert {
                onok: move |_| {
                    err_msg.set(String::new());
                },
                "{err_msg}"
            }
        }
    }
}

#[server]
async fn upload_server(files: Vec<File>) -> Result<Vec<String>, ServerFnError> {
    use crate::models::Image;
    use crate::{get_db, B2, CFG};
    use sha2::{Digest, Sha256};

    let mut rst = vec![];

    let db = get_db().await;

    let total_file_size: usize = files.iter().map(|f| f.data.len()).sum();
    if total_file_size > MAX_FILE_SIZE {
        return Err(ServerFnError::ServerError("文件过大".to_string()));
    }
    for file in &files {
        if !file.is_allow() {
            return Err(ServerFnError::ServerError("不支持的文件格式".to_string()));
        }
        if !file.is_size() {
            return Err(ServerFnError::ServerError("文件过大".to_string()));
        }

        let mut hasher = Sha256::new();
        hasher.update(&file.data);
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);

        let image: Option<Image> = sqlx::query_as(
            r#"SELECT "id","hash","file_path","file_size" FROM "images" WHERE "hash" = $1"#,
        )
        .bind(&hash)
        .fetch_optional(db)
        .await?;

        if let Some(img) = image {
            rst.push(img.file_path.clone());
            continue;
        }

        let path = format!("{}/{}", hash[0..1].to_string(), hash[1..2].to_string());
        let path = format!("{}/{}.{}", path, hash, file.ext_name());

        let resp = B2
            .put_object_with_content_type(&path, &file.data, &file.content_type)
            .await?;

        if resp.status_code() != 200 {
            return Err(ServerFnError::ServerError("上传到对象存储失败".to_string()));
        }

        let image = Image {
            id: xid::new().to_string(),
            hash,
            file_path: path,
            file_size: file.data.len() as i64,
        };
        sqlx::query(
            r#"INSERT INTO "images" ("id", "hash","file_path","file_size") VALUES ($1,$2,$3,$4)"#,
        )
        .bind(&image.id)
        .bind(&image.hash)
        .bind(&image.file_path)
        .bind(image.file_size)
        .execute(db)
        .await?;
        rst.push(image.file_path.clone());
    }
    Ok(rst
        .into_iter()
        .map(|f| format!("{}/{}", &CFG.url_prefix, f))
        .collect())
}
