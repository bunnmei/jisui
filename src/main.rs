use std::{fs, fs::OpenOptions, str};
use std::path::Path;
use std::io::{Read, Write};

use tokio::{
    fs::File,
    io::AsyncWriteExt
};

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    response::{Html, IntoResponse},
    routing::get, 
    extract::Path as axumPath,
    Json, 
    Router
};

use uuid::Uuid;

use serde::{Serialize ,Deserialize};
use scraper::Selector;
use tower_http::services::ServeDir;


mod folder;
use folder::folder::Folder;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("all_pdfs");

    let app = Router::new()
        .route("/", get(root).post(reg_folder))
        .route("/folder_list", get(folder_list))

        .route("/:id", get(displey))
        
        .route("/add", get(add_pdf).post(add_pdf_file))
        .route("/pdf_list", get(pdf_list))
        .nest_service("/resource", serve_dir)
        //uploadするファイルの大きさの上限を設定(1GiB)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    let file_path = "static/index.html";
    // make_json();
    match fs::read_to_string(file_path) {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>404: File Not Found</h1>".to_string()),
    }
}

async fn reg_folder(body: String) -> String {
    // ファイルに書き込むデータを作成
    let write_data = Folder {
            folder_name: body.to_string(),
            uuid:  Uuid::new_v4().to_string(),
            list: vec![],
    };

    let file_name = "folder.json";
    let path = Path::new(file_name);
    if !path.is_file() { //　ファイルがなかった場合
        let create_file = std::fs::File::create("./folder.json");
        match create_file {
            Ok(mut file) => { //ファイルを作成し、jsonに変換後書き込み
                let json_data = serde_json::to_string_pretty(&vec![write_data]);
                if let Ok(data) = json_data {
                    if let Err(error) = file.write_all(data.as_bytes()) {
                        println!("書き込みに失敗 {}", error);
                    }
                } else {
                    println!("書き込みデータのjsonに変換失敗");
                }
            },
            Err(error) => {
                println!("./folder.jsonのファイル作成に失敗。-{}", error)
            }
        }
    } else { //ファイルがあった場合
        // let open_file = std::fs::File::open(path); //普通のFile::openはreadonlyになる
        let open_file = OpenOptions::new().read(true).open(path);
        match open_file {
            Ok(mut file) => {
                let mut contents = String::new(); //ファイルから読み取ったデータの保管場所
                if file.read_to_string(&mut contents).is_ok() {
                    let json_arr= serde_json::from_str::<Vec<Folder>>(&contents);
                    if let Ok(mut arr) = json_arr {
                        arr.push(write_data); //新しいデータの追加
                        if let Ok(string) = serde_json::to_string_pretty(&arr){
                            let mut update_file = fs::File::create(path).expect("update_fileの失敗");
                            if let Err(error) = update_file.write_all(string.as_bytes()) {
                                println!("書き込みに失敗 {}", error);
                            }
                        } else {
                            println!("シリアライズに失敗")
                        }
                    } else {
                        println!("デシリアライズに失敗")
                    }
                } else {
                    println!("ファイルの読み取りに失敗");
                }
            },
            Err(error) => {
                println!("./folder.jsonのファイルを開けませんでした。{}", error);
            }
        }
    }   
    "OK".to_string()
}

async fn folder_list() -> Json<Vec<Folder>> {
    let mut arr:Vec<Folder> = vec![];
    let file_name = "folder.json";
    let path = Path::new(file_name);
    if !path.is_file() {
        return Json(arr);
    }

    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                eprintln!("Failed to read meta.json: {}", e);
            } else {
                match serde_json::from_str::<Vec<Folder>>(&contents) {
                    Ok(folders) => {
                        // println!("Parsed BookData: {:?}", book_data);
                        arr = folders;
                    }
                    Err(e) => eprintln!("Failed to parse meta.json: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Failed to open meta.json: {}", e),
    }

    Json(arr)
}

async fn displey(axumPath(id): axumPath<String>) -> impl IntoResponse {
    // make_json();
    println!("{}", id);
    let file_path = "static/list.html";
    match fs::read_to_string(file_path) {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>404: File Not Found</h1>".to_string()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BookData {
    id: String,
    title: String,
    img: String,
}

#[derive(Serialize)]
struct BookDataList {
    list: Vec<BookData>
}

async fn pdf_list() -> Json<BookDataList> {
    // let data = amazon_scraper().await;
    // match data {
    //     Ok((src, alt)) => {
    //         println!("{}----{}", src, alt);
    //         img_file_download(&src).await;
    //     },
    //     Err(err) => {
    //         println!("error {}", err);
    //     }
    // }
    Json(make_json())
}

async fn add_pdf() -> impl IntoResponse {
    let file_path = "static/add.html";
    // make_json();
    match fs::read_to_string(file_path) {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>404: File Not Found</h1>".to_string()),
    }
}

async fn add_pdf_file(mut multipart: Multipart) -> String {
    let mut text_data = String::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("textData") {
            text_data = field.text().await.unwrap();
            println!("{}", text_data);
        } else if field.name() == Some("pdfFile") {
            // PDFファイルを保存する処理
            let file_name = field.file_name().unwrap_or("temp.pdf");
            let mut file = File::create(format!("./{}", file_name)).await.unwrap();
            
            let mut data = Vec::new(); // バイトデータを格納するベクトル

            while let Some(chunk) = field.chunk().await.unwrap() {
                file.write_all(&chunk).await.unwrap();
                data.extend_from_slice(&chunk);
            }
            println!("ファイルサイズ: {}", data.len());
            println!("保存処理完了")
        }
    }

    "ok".to_string()
}

async fn img_file_download(url: &str) {
    let res = reqwest::get(url).await.unwrap();
    let bytes = res.bytes().await.unwrap();
    let mut file = File::create(format!("./{}", "hoge.jpg")).await.unwrap();
    file.write_all(&bytes).await.unwrap();
}

async fn amazon_scraper () -> Result<(String, String), String>{
    let body = reqwest::get("https://www.amazon.co.jp/%E3%82%AD%E3%83%A5%E3%82%A6%E3%83%AA%E3%81%AE%E3%83%88%E3%82%B2%E3%81%AF%E3%81%AA%E3%81%9C%E6%B6%88%E3%81%88%E3%81%9F%E3%81%AE%E3%81%8B%E2%80%95%E3%82%B5%E3%83%97%E3%83%A9%E3%82%A4%E3%82%BA%E3%81%AA%E3%80%8C%E9%87%8E%E8%8F%9C%E5%AD%A6%E3%80%8D-%E5%AD%A6%E7%A0%94%E6%96%B0%E6%9B%B8-%E8%97%A4%E7%94%B0-%E6%99%BA/dp/4054034608").await.unwrap().text().await.unwrap();
    // println!("{:?}", body);
    let doc = scraper::Html::parse_document(&body);
    // // println!("{:?}", doc);
    // let selector = Selector::parse("span").unwrap();
    // let el = doc.select(&selector).find(|ele| ele.value().attr("id") == Some("productTitle"));
    // if let Some(element) = el {
    //     println!("found element: {}", element.inner_html());
    // } else {
    //     println!("did not find element");
    // }
    let selector = Selector::parse("img").unwrap();
    let el = doc.select(&selector).find(|ele| ele.attr("id") == Some("landingImage"));
    if let Some(element) = el {
        let src_attr_src = element.value().attr("src").unwrap();
        println!("found img_url: {}", src_attr_src);
        let src_attr_alt = element.value().attr("alt").unwrap();
        println!("found title: {}", src_attr_alt);
        //コピーが発生してしまう？
        Ok((src_attr_src.to_string(), src_attr_alt.to_string()))
    } else {
        println!("did not find element");
        Err("fail scraper title and img".to_string())
    }


}

fn make_json() -> BookDataList {
    let mut list: Vec<BookData> = vec![];

    let all_dir_path = Path::new("./all_pdfs");

    // ディレクトリが存在するか確認
    if !all_dir_path.is_dir() {
        eprintln!("The specified path './all' is not a directory or does not exist.");
    }

    // ディレクトリ内のエントリを読み取る
    match fs::read_dir(all_dir_path) {
        Ok(entries) => {
            println!("Directories inside './all_pdfs':");

            // エントリをイテレート
            entries.for_each(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    // ディレクトリかどうか確認
                    if path.is_dir() {
                        if let Some(dir_name) = path.file_name() {
                            println!("{}", dir_name.to_string_lossy());

                            let meta_file_path = path.join("meta.json");
                            
                            if meta_file_path.exists() {
                                match std::fs::File::open(&meta_file_path) {
                                    Ok(mut file) => {
                                        let mut contents = String::new();
                                        if let Err(e) = file.read_to_string(&mut contents) {
                                            eprintln!("Failed to read meta.json: {}", e);
                                        } else {
                                            match serde_json::from_str::<BookData>(&contents) {
                                                Ok(book_data) => {
                                                    println!("Parsed BookData: {:?}", book_data);
                                                    list.push(book_data);
                                                }
                                                Err(e) => eprintln!("Failed to parse meta.json: {}", e),
                                            }
                                        }
                                    }
                                    Err(e) => eprintln!("Failed to open meta.json: {}", e),
                                }
                            }
                        }
                    }
                }
            });
        }
        Err(e) => eprintln!("Failed to read directory: {}", e),
    }
    
    BookDataList {list}
}

// fn read_json_file<T>(file_path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
//     let mut contents = String::new();
//     fs::read_to_string(file_path)?.read_to_string(&mut contents)?;
//     serde_json::from_str(&contents)
// }

// fn write_json_file<T>(file_path: &str, data: &[T]) -> Result<(), Box<dyn std::error::Error>> {
//     let serialized = serde_json::to_string_pretty(data)?;
//     fs::write(file_path, serialized.as_bytes())?;
//     Ok(())
// }