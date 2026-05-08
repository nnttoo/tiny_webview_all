mod webfun;

#[unsafe(no_mangle)]
pub extern "C" fn hello_world() {

    let mystr = "mystr aja dulu ya";

    let my_fun =  move || {
         println!("{}", mystr)
    };
 

    let myconfig = webfun::WebArg {
        url: String::from("helooo"),
        mycb: Box::new(move || {
            my_fun();
        })
    };
    webfun::open_web(&myconfig);
}

#[unsafe(no_mangle)]
pub extern "C" fn tambah(a: i32, b: i32) -> i32 {
    a + b
}
