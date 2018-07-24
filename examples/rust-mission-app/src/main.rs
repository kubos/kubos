// Required by the app_main! macro
extern crate getopts;

#[macro_use]
extern crate kubos_app;

use kubos_app::*;

struct MyApp;

impl AppHandler for MyApp {
    fn on_boot(&self) {
        println!("OnBoot logic");
    }
    fn on_command(&self) {
        println!("OnCommand logic");
    }
}

fn main() {
    let app = MyApp;
    app_main!(&app);
}
