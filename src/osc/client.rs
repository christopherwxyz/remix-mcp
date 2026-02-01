//! Async OSC client for communicating with Ableton Live via `AbletonOSC`.

use std::net::SocketAddr;
use std::time::Duration;

use rosc::{OscMessage, OscPacket, OscType, decoder, encoder};
use tokio::net::UdpSocket;
use tracing::{debug, trace};

use crate::error::Error;
use crate::osc::response::FromOsc;

/// Default port that `AbletonOSC` listens on.
const ABLETON_OSC_PORT: u16 = 11000;

/// Default port that `AbletonOSC` sends responses to.
const RESPONSE_PORT: u16 = 11001;

/// Default timeout for waiting for responses.
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(500);

/// Async OSC client for communicating with Ableton Live.
pub struct OscClient {
    /// Socket for sending OSC messages.
    send_socket: UdpSocket,
    /// Socket for receiving OSC responses.
    recv_socket: UdpSocket,
    /// Address of `AbletonOSC` server.
    ableton_addr: SocketAddr,
    /// Timeout for waiting for responses.
    response_timeout: Duration,
    /// The port we're listening on for responses.
    response_port: u16,
}

impl OscClient {
    /// Create a new OSC client binding to the default response port (11001).
    pub async fn new() -> Result<Self, Error> {
        Self::with_response_port(RESPONSE_PORT).await
    }

    /// Create a new OSC client with a specific response port.
    /// Use port 0 to bind to any available port (useful for tests).
    pub async fn with_response_port(port: u16) -> Result<Self, Error> {
        // Bind send socket to any available port
        let send_socket = UdpSocket::bind("127.0.0.1:0").await?;

        // Bind receive socket to the specified port (0 = any available)
        let recv_socket = UdpSocket::bind(format!("127.0.0.1:{port}")).await?;
        let response_port = recv_socket.local_addr()?.port();

        let ableton_addr: SocketAddr = format!("127.0.0.1:{ABLETON_OSC_PORT}").parse().unwrap();

        debug!(
            send_port = send_socket.local_addr()?.port(),
            recv_port = response_port,
            "OSC client initialized"
        );

        Ok(Self {
            send_socket,
            recv_socket,
            ableton_addr,
            response_timeout: DEFAULT_TIMEOUT,
            response_port,
        })
    }

    /// Get the response port this client is listening on.
    pub fn response_port(&self) -> u16 {
        self.response_port
    }

    /// Tell `AbletonOSC` to send responses to this client's response port.
    /// This must be called before queries if using a non-default port.
    pub async fn configure_response_port(&self) -> Result<(), Error> {
        if self.response_port != RESPONSE_PORT {
            self.send(
                "/live/api/respond_to",
                vec![OscType::Int(i32::from(self.response_port))],
            )
            .await?;
            // Give Ableton a moment to process
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok(())
    }

    /// Send an OSC message without waiting for a response.
    pub async fn send(&self, addr: &str, args: Vec<OscType>) -> Result<(), Error> {
        let msg = OscMessage {
            addr: addr.to_string(),
            args,
        };
        let packet = OscPacket::Message(msg);
        let bytes = encoder::encode(&packet)?;

        trace!(address = addr, "Sending OSC message");
        self.send_socket.send_to(&bytes, self.ableton_addr).await?;

        Ok(())
    }

    /// Send an OSC message and wait for a response.
    pub async fn query<T: FromOsc>(&self, addr: &str, args: Vec<OscType>) -> Result<T, Error> {
        // Clear any pending messages in the receive buffer
        self.clear_recv_buffer().await;

        // Send the query
        self.send(addr, args).await?;

        // Wait for response
        let mut buf = [0u8; 65536];
        let (len, _src) =
            tokio::time::timeout(self.response_timeout, self.recv_socket.recv_from(&mut buf))
                .await??;

        let (_, packet) = decoder::decode_udp(&buf[..len])?;
        trace!(?packet, "Received OSC response");

        T::from_osc(packet)
    }

    /// Send an OSC message and collect multiple responses until timeout.
    #[allow(dead_code)]
    pub async fn query_all(&self, addr: &str, args: Vec<OscType>) -> Result<Vec<OscPacket>, Error> {
        // Clear any pending messages
        self.clear_recv_buffer().await;

        // Send the query
        self.send(addr, args).await?;

        // Collect responses until timeout
        let mut responses = Vec::new();
        let mut buf = [0u8; 65536];

        while let Ok(Ok((len, _src))) =
            tokio::time::timeout(self.response_timeout, self.recv_socket.recv_from(&mut buf)).await
        {
            if let Ok((_, packet)) = decoder::decode_udp(&buf[..len]) {
                responses.push(packet);
            }
        }

        Ok(responses)
    }

    /// Clear any pending messages in the receive buffer.
    async fn clear_recv_buffer(&self) {
        let mut buf = [0u8; 1024];
        while tokio::time::timeout(
            Duration::from_millis(1),
            self.recv_socket.recv_from(&mut buf),
        )
        .await
        .is_ok()
        {}
    }

    /// Test connection to Ableton Live.
    pub async fn test_connection(&self) -> Result<bool, Error> {
        // Send a simple query to check if Ableton is responding
        match self
            .query::<Vec<OscType>>("/live/song/get/tempo", vec![])
            .await
        {
            Ok(_) => Ok(true),
            Err(Error::Timeout) => Ok(false),
            Err(e) => Err(e),
        }
    }
}
