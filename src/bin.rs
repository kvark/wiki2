#[macro_use] extern crate nickel;
extern crate hyper; 
extern crate wiki2; 
extern crate clap;
#[macro_use] extern crate log; 

use std::path::{Path}; 
use std::io;
use std::io::prelude::*;
use std::fs::File; 

use nickel::{Nickel, HttpRouter};
use nickel::status::{StatusCode};

use clap::{App, Arg}; 
 
use wiki2::render_html; 


fn run(base_path : &String, host : &str) -> () {
	let mut server = Nickel::new();
    let base_path = Path::new("./tests/test1"); 
    let mut router = Nickel::router();
    
    router.get("**", middleware! { |request| 
		info!("{:?}", request.origin.uri); 
		let uri = &request.origin.uri;
		match uri {
			&hyper::uri::RequestUri::AbsolutePath(ref request_path) => {
				let request_path = ".".to_owned() + request_path; // make the absolute path relative to where i'm at
				let mut path2 = base_path.join(Path::new(&request_path));
				if path2.is_dir() {
					path2 = path2.join("_index.md"); 
				}
				match File::open(path2) {
					Ok(mut file) => {
						let mut mdText = String::new();
						match file.read_to_string(&mut mdText) {
							Ok(_) => {
								let mut htmlText = render_html(&*mdText);
								( StatusCode::Ok, format!("{}", htmlText) )
							}
							_ => (StatusCode::BadRequest, "could not read file".into() )
						}
					}
					_ => {
						(StatusCode::NotFound, "no such file".into())
					}
				}
			},
			_ => {
				warn!("unusual uri type requested {}", uri); 
				(StatusCode::BadRequest, "uri type not supported".into() )
			},
		}
    });
    server.utilize(router);
    server.listen(host);
}

fn main() {
	let matches = App::new("MyApp")
		.version("1.0")
		.author("Wayne Nilsen <waynenilsen@gmail.com>")
		.about("Hosts a website by parsing markdown files. ")
		.arg(Arg::with_name("base-dir")
			.short("d")
			.long("base-dir")
			.help("base directory to host the pages from")
			.takes_value(true))
		.arg(Arg::with_name("host")
			.short("h")
			.long("host")
			.help("where to listen (localhost:80)")
			.takes_value(true))
		.get_matches();
		
	run(&matches.value_of("base-dir").unwrap_or(".").into()
		,&matches.value_of("host").unwrap_or("0.0.0.0:80"));
}
