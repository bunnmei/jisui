use std::fs::File;
use std::io::Write;
use scraper::Selector;
use std::error::Error;
use std::path::Path;

pub async  fn get_amazon_img(url: &str) -> Option<(File, String)> {
  let body = reqwest::get(url).await.unwrap().text().await.unwrap();
  let doc = scraper::Html::parse_document(&body);
  let selector = Selector::parse("img").unwrap();
  let el = doc.select(&selector).find(|ele| ele.attr("id") == Some("landingImage"));
  if let Some(element) = el {
      let src_attr_src = element.value().attr("src").unwrap();
      println!("found img_url: {}", src_attr_src);
      let src_attr_alt = element.value().attr("alt").unwrap();
      println!("found title: {}", src_attr_alt);
      let filename = src_attr_src.split('/').last().unwrap();
      let res = reqwest::get(src_attr_src).await.unwrap();
      let bytes = res.bytes().await.unwrap();
      
      let mut file = File::create(format!("./{}", filename)).unwrap();
      file.write_all(&bytes).unwrap();
      
      //コピーが発生してしまう？
      Some((file, src_attr_alt.to_string()))
  } else {
      println!("did not find element");
      None
  }  
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