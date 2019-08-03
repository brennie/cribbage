// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::net::*;

use structopt::StructOpt;
use trust_dns_resolver::config::{Protocol, ResolverConfig, ResolverOpts, NameServerConfig};
use trust_dns_resolver::Resolver;
use trust_dns_server::server;

pub mod card;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "list")]
    List,

    #[structopt(name = "serve")]
    Serve,

    #[structopt(name = "connect")]
    Connect,
}

fn main() {
    let cmd = Command::from_args();

    match cmd {
        Command::List => list(),
        Command::Serve => serve(),
        _ => unimplemented!(),
    }

    println!("{:?}", cmd);
}



fn list() {

    let resolver = {
        let mut config = ResolverConfig::new();
        config.add_name_server(
            NameServerConfig {
                socket_addr: trust_dns_resolver::proto::multicast::MDNS_IPV4,
                protocol: Protocol::Mdns,
                tls_dns_name: None,
            });

        Resolver::new(config, ResolverOpts::default())
            .unwrap()
    };
}

