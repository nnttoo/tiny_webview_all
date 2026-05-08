pub struct  WebArg{
    pub url : String,   
    pub mycb : Box<dyn Fn()>
}

pub fn open_web(myconfig: &WebArg){
    println!("Hallo dari Rust mod fnunnnn!");
    println!("Menjalankan: {}", myconfig.url);
    (myconfig.mycb)();
}

