extern crate gst;
extern crate gstreamer_sys;

fn main() {
    gst::init();

    let mut pipeline = gst::Pipeline::new("pipe").unwrap();
    let videosrc = gst::Element::new("autovideosrc", "src").unwrap();
    let videosink = gst::Element::new("autovideosink", "sink").unwrap();

    if !pipeline.add_and_link(videosrc, videosink) {
        panic!("couldn't link videosrc and videosink");
    }

    let bus_receiver = pipeline.bus().unwrap().receiver();

    let mut mainloop = gst::MainLoop::new();
    mainloop.spawn();
    pipeline.play();
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
