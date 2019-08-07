// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use structopt::StructOpt;

pub mod card;
pub mod net;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "list")]
    List,

    #[structopt(name = "serve")]
    Serve(ServeOptions),

    #[structopt(name = "connect")]
    Connect,
}

#[derive(Debug, StructOpt)]
struct ServeOptions {
    #[structopt(name = "name")]
    name: String,

    #[structopt(long, default_value = "2929")]
    port: u16,

    #[structopt(long)]
    password: Option<String>,
}

fn main() {
    let cmd = Command::from_args();

    match cmd {
        Command::Serve(options) => serve(options),
        Command::List => list(),
        _ => unimplemented!(),
    }
}

fn serve(options: ServeOptions) {
    let adv = net::serve_advertisement(options.name.clone(), options.port, options.password.is_some());

    tokio::run(adv);
}

fn list() {
    let adv = net::query_advertisements();
    tokio::run(adv);
}
