//! API shim to make it nicer to interact with the ockam messaging API

use minicbor::Decoder;
use ockam::Result;
use ockam_api::{
    nodes::types::{
        CreateTransport, NodeStatus, TransportList, TransportMode, TransportStatus, TransportType,
    },
    Method, Request, Response,
};

////////////// !== generators

/// Construct a request to query node status
pub(crate) fn query_status() -> Result<Vec<u8>> {
    let mut buf = vec![];
    Request::builder(Method::Get, "/node").encode(&mut buf)?;
    Ok(buf)
}

/// Construct a request to query node transports
pub(crate) fn query_transports() -> Result<Vec<u8>> {
    let mut buf = vec![];
    Request::builder(Method::Get, "/node/transport").encode(&mut buf)?;
    Ok(buf)
}

/// Construct a request to query node transports
pub(crate) fn create_transport(cmd: &crate::transport::CreateCommand) -> Result<Vec<u8>> {
    let payload = CreateTransport::new(
        TransportType::Tcp,
        if cmd.connect {
            TransportMode::Connect
        } else {
            TransportMode::Listen
        },
        &cmd.address,
    );

    let mut buf = vec![];
    Request::builder(Method::Post, "/node/transport")
        .body(payload)
        .encode(&mut buf)?;
    Ok(buf)
}

////////////// !== parsers

/// Parse the returned status response
pub(crate) fn parse_status(resp: &[u8]) -> Result<NodeStatus> {
    let mut dec = Decoder::new(resp);
    let _ = dec.decode::<Response>()?;
    Ok(dec.decode::<NodeStatus>()?)
}

/// Parse the returned status response
pub(crate) fn parse_transport_list(resp: &[u8]) -> Result<TransportList> {
    let mut dec = Decoder::new(resp);
    let _ = dec.decode::<Response>()?;
    Ok(dec.decode::<TransportList>()?)
}

/// Parse the returned status response
pub(crate) fn parse_transport_status(resp: &[u8]) -> Result<(Response, TransportStatus<'_>)> {
    let mut dec = Decoder::new(resp);
    let response = dec.decode::<Response>()?;
    Ok((response, dec.decode::<TransportStatus>()?))
}