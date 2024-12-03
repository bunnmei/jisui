// use std::fs::File;
use tokio::{fs::File, io::AsyncWriteExt};
use scraper::Selector;
use std::path::Path;

pub async fn get_amazon_img(url: &str, path: &Path) -> Option<(String, String)> {

  let mut img = String::new();
  let mut name = String::new();
  let mut title = String::new();
  {
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    
    let doc = scraper::Html::parse_document(&body);
    let selector = Selector::parse("img").unwrap();
    let el = doc.select(&selector).find(|ele| ele.attr("id") == Some("landingImage"));
    match el {
      Some(element) => {
        let src_attr_src = element.value().attr("src");
            println!("found img_url: {}", src_attr_src.unwrap());
            let src_attr_alt = element.value().attr("alt").unwrap();
            println!("found title: {}", src_attr_alt);
            title = src_attr_alt.to_string();
            if let Some(addr) = src_attr_src {
              img = addr.to_string();
              let filename = addr.split('/').last().unwrap();
              name = filename.to_string();
            }
          },
          None => {
            return None
          }
        }
  }
  {
      let res = reqwest::get(&img).await.unwrap();
      let bytes = res.bytes().await.unwrap();
      let mut file = File::create(path.join(&name)).await.unwrap();
      file.write_all(&bytes).await.unwrap();

      Some((name, title))
  }
  // Some(("hoge.hog".to_string(), "hoge".to_string()))
  // if let Some(element) = el {
    //     let src_attr_src = element.value().attr("src").unwrap();
  //     println!("found img_url: {}", src_attr_src);
  //     let src_attr_alt = element.value().attr("alt").unwrap();
  //     println!("found title: {}", src_attr_alt);
  //     let filename = src_attr_src.split('/').last().unwrap();
  //     let res = reqwest::get(src_attr_src).await.unwrap();
  //     let bytes = res.bytes().await.unwrap();
      
  //     let mut file = File::create(path.join(filename)).unwrap();
  //     file.write_all(&bytes).unwrap();

  //     //コピーが発生してしまう？
  //     Some((filename.to_string(), src_attr_alt.to_string()))
  // } else {
  //     println!("did not find element");
  //     Some(("hoge.hog".to_string(), "hoge".to_string()))
  // }  
}

// pub fn search_pdfs(id: &str) -> Result<Vec<T>, Box<dyn Error>> {
//   let path = format!( "{}/{}", "PDFs", id);
//   let folder_path = Path::new(&path);
//   if folder_path.exists() {
      
//   } else {
//     match fs::create_dir(folder_path) {
//         Ok(_) => println!("フォルダパスがつくられました。"),
//         Err(e) => eprintln!("フォルダ作成失敗 {}", e)
//     }
//   }
// }

// fn search_pdf_list(path: &Path) {
//     match fs::read_dir(path) {
//         Ok(dir) => {
//           dir.for_each(|entry|{
//             if let Ok(ent) = entry {
//               let entry_path = ent.path();

//               if entry_path.is_dir() {
//                 let mata_file_path = path.join("meta.json");
//               }
//             }
//           });
//         },
//         Err(e) => {
//             println!("ディレクトリをよみとれません。{}", e)
//         }
//     }
// }