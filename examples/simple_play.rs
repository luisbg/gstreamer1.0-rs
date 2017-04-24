extern crate gst;

use std::env;

fn main() {
    gst::init();

    let args: Vec<String> = env::args().collect();
    let uri = if args.len() == 2 {
        gst::filename_to_uri(args[1].as_ref()).unwrap()
    } else {
        panic!("Usage: playbin file_path");
    };

    let mut playbin = gst::PlayBin::new("player").unwrap();
    playbin.set_uri(uri.as_ref());
    let bus_receiver = playbin.bus().unwrap().receiver();

    let mut mainloop = gst::MainLoop::new();
    mainloop.spawn();
    playbin.play();
    for message in bus_receiver.iter() {
        match message.parse() {
            gst::Message::Eos(_) => {
                println!("Eos received");
                break;
            }
            _ => {}
        }
    }

    mainloop.quit();
}
