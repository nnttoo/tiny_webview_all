use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub struct FileType{
   pub file_name : String,
   pub ext : Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct FileSelectorArg{
   pub root_dir : String,
   pub file_types : Vec<FileType>
}
