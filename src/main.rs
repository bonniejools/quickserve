//use std::net::{TcpListener, TcpStream};
extern crate thrussh;
extern crate futures;
extern crate tokio_core;
extern crate env_logger;
extern crate iron;
use std::sync::Arc;
use thrussh::*;
use std::io;
use thrussh::server::Response as sshResponse;

#[allow(unused_imports)]
use iron::prelude::*;
#[allow(unused_imports)]
use iron::status;

#[derive(Clone)]
struct H{}

impl server::Handler for H {
    type Error = ();
    type FutureAuth = futures::Finished<(Self, server::Auth), Self::Error>;
    type FutureUnit = futures::Finished<(Self, server::Session), Self::Error>;
    type FutureBool = futures::Finished<(Self, server::Session, bool), Self::Error>;

    //fn auth_publickey(self, _: &str, _: &key::PublicKey) -> Self::FutureAuth {
        //println!("Testing auth...");
        //futures::finished((self, server::Auth::Accept))
    //}
    fn auth_keyboard_interactive(self,user:&str,submethods:&str, response:Option<sshResponse>)-> Self::FutureAuth{
        println!("{:?}", response);
        futures::finished((self, server::Auth::Accept))
    }

    fn auth_none(self, user: &str) -> Self::FutureAuth {
        println!("Testing none :))))");
        futures::finished((self, server::Auth::Accept))
    }

    //fn auth_password(self, user: &str, password: &str) -> Self::FutureAuth {
        //println!("Testing password {}", password);
        //futures::finished((self, server::Auth::Accept))
    //}

    fn data(mut self, channel: ChannelId, data: &[u8], mut session: server::Session) -> Self::FutureUnit {
        println!("data on channel {:?}: {:?}", channel, std::str::from_utf8(data));
        session.data(channel, None, data).unwrap();
        futures::finished((self, session))
    }
}

fn runSSHServer() {
    env_logger::init().unwrap();
    // Starting the server thread.
    let t = std::thread::spawn(|| {
        let mut config = thrussh::server::Config::default();

        for (index, flag) in config.methods.enumerate() {
            if index != 4 {
                config.methods.remove(flag);
            }
        }

        println!("{:?}", config.methods);

        config.connection_timeout = Some(std::time::Duration::from_secs(600));
        config.auth_rejection_time = std::time::Duration::from_secs(3);
        config.keys.push(thrussh::key::Algorithm::generate_keypair(thrussh::key::ED25519).unwrap());

        let config = Arc::new(config);
        let sh = H{};
        thrussh::server::run(config, "0.0.0.0:2222", sh);
    });
}

fn start_web() 
{
    Iron::new(|_:&mut Request|
              {
                  Ok(Response::with((status::Ok, "Hello!\r\n")))
              }).http("localhost:8000").unwrap();
}

fn main() {
    //startWeb();
    runSSHServer();
    start_web();
    let mut x = String::new();
    io::stdin().read_line(&mut x);
}

