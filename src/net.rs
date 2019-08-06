// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use bincode;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio::codec::LengthDelimitedCodec;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::*;

const IP_ALL: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const IP_MULTICAST: Ipv4Addr = Ipv4Addr::new(229, 29, 29, 29);
const ADVERT_PORT: u16 = 29999;

fn multicast_udp_socket(
    local_addr: &SocketAddrV4,
) -> io::Result<std::net::UdpSocket> {
    use socket2::{Domain, Protocol, SockAddr, Socket, Type};

    let socket = Socket::new(Domain::ipv4(), Type::dgram(), Some(Protocol::udp()))?;

    socket.set_reuse_address(true)?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(&IP_MULTICAST, &local_addr.ip())?;
    socket.bind(&SockAddr::from(*local_addr))?;

    Ok(socket.into_udp_socket())
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct AdvertisementResponse {
    magic: &'static str,
    port: u16,
}

const MAGIC_REQUEST: &'static str = "cribbage-advertisement-request";
const MAGIC_RESPONSE: &'static str = "cribbage-advertisement-response";

pub fn serve_advertisement(port: u16) -> impl Future<Item = (), Error = ()> {
    let socket = UdpSocket::from_std(
        multicast_udp_socket(&SocketAddrV4::new(IP_ALL, ADVERT_PORT)).unwrap(),
        &tokio::reactor::Handle::default(),
    )
    .expect("Could not convert to tokio socket?");

    let (tx, rx) = UdpFramed::new(socket, LengthDelimitedCodec::new()).split();

    future::loop_fn((tx, rx), move |(tx, rx)| {
        rx.into_future()
            .map_err(drop)
            .and_then(move |(request, rx)| {
                if let Some((bytes, addr)) = request {
                    if bytes == MAGIC_REQUEST {
                        let rsp = Bytes::from(
                            bincode::serialize(&AdvertisementResponse {
                                magic: MAGIC_RESPONSE,
                                port: port,
                            })
                            .unwrap(),
                        );

                        future::Either::A(
                            tx.send((rsp, addr))
                                .map_err(drop)
                                .map(move |tx| future::Loop::Continue((tx, rx))),
                        )
                    } else {
                        future::Either::B(future::ok(future::Loop::Continue((tx, rx))))
                    }
                } else {
                    drop(tx.reunite(rx).unwrap());
                    future::Either::B(future::ok(future::Loop::Break(())))
                }
            })
    })
}

pub fn query_advertisements() -> impl Future<Item = (), Error = ()> {
    let local_addr = SocketAddrV4::new(IP_ALL.into(), 1234);
    let multicast_addr = SocketAddr::new(IpAddr::V4(IP_MULTICAST), ADVERT_PORT);

    let socket = UdpSocket::from_std(
        multicast_udp_socket(&local_addr).unwrap(),
        &tokio::reactor::Handle::default()
    ).unwrap();

    let (tx, rx) = UdpFramed::new(socket, LengthDelimitedCodec::new()).split();

    tx.send((Bytes::from(MAGIC_REQUEST), multicast_addr))
        .map_err(drop)
        .and_then(move |_tx| {
            future::loop_fn(rx, |rx| {
                rx
                    .into_future()
                    .map_err(drop)
                    .map(|(rsp, rx)| {
                        if let Some((rsp, addr)) = rsp {
                            match bincode::deserialize::<AdvertisementResponse>(&rsp) {
                                Ok(a) => drop(a),
                                Err(e) => drop(e),
                            }

                            future::Loop::Continue(rx)
                        } else {
                            future::Loop::Break(())
                        }
                    })
            })
        })
        .map(drop)
}