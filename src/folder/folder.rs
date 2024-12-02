use std::{fs::OpenOptions, io::Read, fs::File};
use std::error::Error;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub folder_name: String,
    pub uuid: String,
    pub list: Vec<String>,
}

pub fn read_json_file<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> 
where
    T: DeserializeOwned,
{
    // let open_file = OpenOptions::new().read(true).open(file_path);
    let open_file = File::open(file_path);
    match open_file {
        Ok(mut file) => {
            let mut contents = String::new();
            let read_file = file.read_to_string(&mut contents);
            match read_file {
                Ok(_) => {
                    let res =  serde_json::from_str::<Vec<T>>(&contents);
                    match res {
                        Ok(vec) => {
                            Ok(vec)
                        },
                        Err(err) => {
                            println!("デシリアライズに失敗 - {}", err);
                            Err(Box::new(err))
                        }
                    }
                    },
                Err(err) => {
                    println!("fileの読み取りに失敗 - {}", err);
                    Err(Box::new(err))
                }
            }
        }
        Err(err) => {
            println!("ファイルを開くことができませんでした。- {}", err);
            Err(Box::new(err))
        }
    }
}

pub fn json_file_search_uuid(json: Result<Vec<Folder>, Box<dyn Error>>, id:&str) ->  Option<String> {
    match json {
        Ok(vec) => {
            let find_folder = vec.iter().find(|folder| folder.folder_name == id);
            match find_folder {
                Some(folder) => {
                    Some(folder.uuid.clone())
                },
                None => {
                    println!("見つかりませんでした。");
                    None
                }
            }
        }, 
        Err(e) => {
            println!("第一引数に結果エラーの変数が入力されています。");
            None
        }
    }
}

