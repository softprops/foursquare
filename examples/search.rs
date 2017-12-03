extern crate env_logger;
extern crate foursquare;
extern crate tokio_core;
#[macro_use(quick_main)]
extern crate error_chain;

use std::env;

use tokio_core::reactor::Core;

use foursquare::{Credentials, Client, Result};

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("FS_CLIENT_ID").ok().and_then(|id| {
        env::var("FS_CLIENT_SECRET").ok().map(|sec| (id, sec))
    }) {
        Some((id, sec)) => {
            let mut core = Core::new()?;
            let foursq = Client::new("20170801", Some(Credentials::new(id, sec)), &core.handle());
            match core.run(foursq.venues().search()) {
                Ok(res) => println!("{:#?}", res),
                Err(err) => println!("err {}", err),
            }
            Ok(())
        }
        _ => Err(
            "example missing FS_CLIENT_ID and/or FS_CLIENT_SECRET".into(),
        ),
    }
}