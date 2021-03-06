extern crate gst;

use std::thread;
use std::process::Command;


fn main(){
    gst::init();
    let pipeline_str = "autoaudiosrc ! audioconvert ! appsink name=appsink0 caps=\"audio/x-raw,format=F32LE,channels=1\"";
    let mut pipeline = gst::Pipeline::new_from_str(pipeline_str).unwrap();
	let mut mainloop = gst::MainLoop::new();
	let mut bus = pipeline.bus().expect("Couldn't get bus from pipeline");
	let bus_receiver = bus.receiver();
	let appsink = pipeline.get_by_name("appsink0").expect("Couldn't get appsink from pipeline");
	let appsink = gst::AppSink::new_from_element(appsink);
	mainloop.spawn();
	pipeline.play();
	Command::new("tput").args(&["civis","--","invisible"]).status().unwrap();
	thread::spawn(move||{
		loop {
			match appsink.recv(){
				Ok(gst::appsink::Message::NewPreroll(sample)) | Ok(gst::appsink::Message::NewSample(sample)) => {
				    if let Some(_) = sample.buffer(){
					}
				}
				Ok(gst::appsink::Message::Eos) => {
					println!("appsink thread received EOS, quitting");
					break;
				}
				Err(_) => {
					println!("appsink thread channel closed, quitting");
					break;
				}
			}
		}
	});

	for message in bus_receiver.iter(){
		match message.parse(){
			gst::Message::StateChangedParsed{ref old, ref new, ..} => {
				println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new);
			}
            gst::Message::ErrorParsed{ref error, ref debug, ..} => {
				println!("error msg from element `{}`: {}, {}. Quitting", message.src_name(), error.message(), debug);
                break;
            }
			gst::Message::Eos(_) => {
				println!("eos received quiting");
				break;
			}
			_ => {
				println!("msg of type `{}` from element `{}`", message.type_name(), message.src_name());
			}
		}
	}
	mainloop.quit();
	Command::new("tput").args(&["cnorm","--","normal"]).status().unwrap();
}
