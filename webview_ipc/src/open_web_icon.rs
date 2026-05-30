use std::{fs, path::PathBuf};
use tao::window::Icon;

use crate::utils_tools::get_exe_folder;




pub fn load_dynamic_png() -> Option<Icon> {

    
    let  path : Option<PathBuf> = (|| {

        let filename = "icon.png";
        let curpath = get_exe_folder().join(filename); 

        fn is_exist(path : &PathBuf)->bool{
            match fs::exists(path){
                Ok(is)=>is,
                _=>false
            }
        }

        if is_exist(&curpath){ 
            return Some(curpath);
        }   

        let curpath : PathBuf = filename.into();
        if is_exist(&curpath) {
            return Some(curpath);
        }

        None
        
    })(); 

    let Some(path) = path else {
        return None;
    };

    

    let Ok(img) = image::open(path) else {
        return None;
    };
    
    let img = img.into_rgba8();  

    let (width, height) = img.dimensions();
    let rgba = img.into_raw(); 
    let Ok(window_icon) = Icon::from_rgba(rgba, width, height) else {
        
        return None;
    
    };
    Some(window_icon)
}