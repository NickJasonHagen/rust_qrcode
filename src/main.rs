use qrcode::QrCode;
use image::Luma;
pub use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let code = QrCode::new(args[1].as_bytes()).unwrap();
        let image = code.render::<Luma<u8>>().build();
        image.save(args[2].to_string()).unwrap();
        println!("done,")
    }
    else{
        println!("error,\n #arguments \"./rust_qrcode http://urlstri.ng ./imagefilepath\"");
    }
}
