use dioxus::prelude::*;

use crate::{components::dialog::Confirm, models};

#[component]
pub fn ImageHome() -> Element {
    let token = use_context::<Signal<String>>();
    let mut page = use_signal(|| 0u32);
    let mut load_image_list =
        use_resource(move || async move { image_list_server(token(), page()).await });

    match &*load_image_list.read_unchecked() {
        Some(Ok(image_list)) => {
            rsx! {
                div { class: "w-full overflow-x-auto",
                    table { class: "w-full",
                        thead {
                            tr { class: "bg-gray-300",
                                th { class: "text-start p-3", "#" }
                                th { class: "text-start p-3", "哈希" }
                                th { class: "text-start p-3", "图片" }
                                th { class: "text-start p-3", "大小" }
                                th { class: "text-start p-3", "操作" }
                            }
                        }
                        tbody {
                            for img in &image_list.data {
                                tr { class: "odd:bg-white hover:ring-inset hover:ring-1 hover:ring-blue-200",
                                    td { class: "font-mono uppercase p-3", "{img.id}" }
                                    td { class: "p-3 max-w-36 lg:max-w-max",
                                        div { class: "truncate font-mono uppercase text-xs",
                                            "{img.hash}"
                                        }
                                    }
                                    td { class: "p-3",
                                        a {
                                            href: "{img.file_path}",
                                            target: "_blank",
                                            img {
                                                src: "{img.file_path}",
                                                class: "w-10 h-10 object-cover",
                                            }
                                        }
                                    }
                                    td { class: "p-3",
                                        div { class: "text-xs", "{img.file_size}" }
                                    }
                                    td { class: "p-3",
                                        DelImg {
                                            id: img.id.clone(),
                                            file_path: img.file_path.clone(),
                                            load_image_list,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                ul { class: "flex justify-end items-center gap-x-2",
                    for i in 1..=image_list.total_page {
                        li { key: "page-{i}",
                            button {
                                class: "p-1  hover:text-blue-600",
                                onclick: move |_| {
                                    page.set(i - 1);
                                    load_image_list.restart();
                                },
                                "{i}"
                            }
                        }
                    }
                }
            }
        }
        Some(Err(e)) => {
            rsx! {
                div { "{e}" }
            }
        }
        None => {
            rsx! {
                div { "加载中..." }
            }
        }
    }
}

#[server]
async fn image_list_server(
    token: String,
    page: u32,
) -> Result<models::Pagination<models::Image>, ServerFnError> {
    use crate::{get_db, CFG};

    let db = get_db().await;

    let count: (i64,) = sqlx::query_as(r#"SELECT count(*) FROM "images""#)
        .fetch_one(db)
        .await?;

    let mut q =
        sqlx::QueryBuilder::new(r#"SELECT "id","hash","file_path","file_size" FROM "images""#);
    q.push(" ORDER BY id DESC");
    q.push(" LIMIT ")
        .push_bind(models::DEFAULT_PAGE_SIZE as i64)
        .push(" OFFSET ")
        .push_bind((page * models::DEFAULT_PAGE_SIZE) as i64);

    let image_list = q.build_query_as().fetch_all(db).await?;
    let image_list = image_list
        .into_iter()
        .map(|i: models::Image| models::Image {
            file_path: format!("{}/{}", CFG.url_prefix, i.file_path),
            ..i
        })
        .collect();

    let p = models::Pagination::with_count(image_list, count, 1);
    Ok(p)
}

#[component]
fn DelImg(
    id: String,
    file_path: String,
    load_image_list: Resource<Result<models::Pagination<models::Image>, ServerFnError>>,
) -> Element {
    let mut confirm_del = use_signal(|| false);
    rsx! {
        button { onclick: move |_| confirm_del.set(true), "删除" }
        if confirm_del() {
            Confirm {
                oncancel: move |_| confirm_del.set(false),
                onok: move |_| {
                    let _id = id.clone();
                    async move {
                        confirm_del.set(false);
                    }
                },
                div { class: "flex flex-col justify-center items-center gap-y-3",
                    div {
                        img { src: "{file_path}", class: "w-12 object-cover" }
                    }
                    div { "删除后不可恢复，确定删除吗？" }
                }
            }
        }
    }
}
