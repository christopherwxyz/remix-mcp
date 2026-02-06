//! Async OSC client for communicating with Ableton Live via `AbletonOSC`.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use rosc::{OscMessage, OscPacket, OscType, decoder, encoder};
use tokio::net::UdpSocket;
use tokio::sync::OnceCell;
use tracing::{debug, trace};

use crate::error::Error;
use crate::osc::response::FromOsc;

/// Default port that `AbletonOSC` listens on.
const ABLETON_OSC_PORT: u16 = 11000;

/// Default timeout for waiting for responses.
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(500);

/// Async OSC client for communicating with Ableton Live.
///
/// Uses a single UDP socket for both sending and receiving. `AbletonOSC` replies
/// to the sender's address, so each client instance automatically receives its
/// own responses on its ephemeral port — no fixed port contention.
pub struct OscClient {
    /// Single socket used for both sending and receiving OSC messages.
    socket: UdpSocket,
    /// Address of `AbletonOSC` server.
    ableton_addr: SocketAddr,
    /// Timeout for waiting for responses.
    response_timeout: Duration,
}

impl OscClient {
    /// Create a new OSC client bound to an ephemeral port.
    pub async fn new() -> Result<Self, Error> {
        let socket = UdpSocket::bind("127.0.0.1:0").await?;
        let ableton_addr: SocketAddr = format!("127.0.0.1:{ABLETON_OSC_PORT}").parse().unwrap();

        debug!(port = socket.local_addr()?.port(), "OSC client initialized");

        Ok(Self {
            socket,
            ableton_addr,
            response_timeout: DEFAULT_TIMEOUT,
        })
    }

    /// Get the local port this client is bound to.
    pub fn local_port(&self) -> u16 {
        self.socket.local_addr().map(|a| a.port()).unwrap_or(0)
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
        self.socket.send_to(&bytes, self.ableton_addr).await?;

        Ok(())
    }

    /// Send an OSC message and wait for a response.
    pub async fn query<T: FromOsc>(&self, addr: &str, args: Vec<OscType>) -> Result<T, Error> {
        // Clear any pending messages in the receive buffer
        self.clear_recv_buffer().await;

        // Send the query
        self.send(addr, args).await?;

        // Wait for response on the same socket we sent from
        let mut buf = [0u8; 65536];
        let (len, _src) =
            tokio::time::timeout(self.response_timeout, self.socket.recv_from(&mut buf)).await??;

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
            tokio::time::timeout(self.response_timeout, self.socket.recv_from(&mut buf)).await
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
        while tokio::time::timeout(Duration::from_millis(1), self.socket.recv_from(&mut buf))
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

/// Lazy wrapper around [`OscClient`] that defers socket binding until first use.
///
/// This allows the MCP server to start and complete the handshake even when
/// Ableton is not running. The underlying [`OscClient`] is created on the
/// first call that needs it.
///
/// Multiple `OscHandle` instances (from different remix-mcp processes) can
/// coexist because each gets its own ephemeral port — `AbletonOSC` replies
/// directly to the sender's address.
#[derive(Clone)]
pub struct OscHandle {
    inner: Arc<OnceCell<OscClient>>,
}

impl Default for OscHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl OscHandle {
    /// Create a new handle. No sockets are opened.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(OnceCell::new()),
        }
    }

    /// Get or lazily initialize the underlying [`OscClient`].
    async fn client(&self) -> Result<&OscClient, Error> {
        self.inner
            .get_or_try_init(|| async {
                let client = OscClient::new().await?;
                debug!("OSC client bound to port {}", client.local_port());
                Ok(client)
            })
            .await
    }

    /// Send an OSC message without waiting for a response.
    pub async fn send(&self, addr: &str, args: Vec<OscType>) -> Result<(), Error> {
        self.client().await?.send(addr, args).await
    }

    /// Send an OSC message and wait for a single typed response.
    pub async fn query<T: FromOsc>(&self, addr: &str, args: Vec<OscType>) -> Result<T, Error> {
        self.client().await?.query(addr, args).await
    }

    /// Send an OSC message and collect multiple responses until timeout.
    pub async fn query_all(&self, addr: &str, args: Vec<OscType>) -> Result<Vec<OscPacket>, Error> {
        self.client().await?.query_all(addr, args).await
    }

    /// Test connection to Ableton Live.
    pub async fn test_connection(&self) -> Result<bool, Error> {
        self.client().await?.test_connection().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `OscHandle::new()` creates no sockets (inner `OnceCell` is empty).
    #[test]
    fn handle_new_does_not_connect() {
        let handle = OscHandle::new();
        assert!(!handle.inner.initialized());
    }

    /// Calling `client()` populates the `OnceCell`.
    #[tokio::test]
    async fn handle_lazily_connects_on_first_access() {
        let handle = OscHandle::new();
        assert!(!handle.inner.initialized());

        let client = handle.client().await;
        assert!(client.is_ok());
        assert!(handle.inner.initialized());
    }

    /// Two `client()` calls return the same port (`OnceCell` caching).
    #[tokio::test]
    async fn handle_returns_same_client_on_repeated_access() {
        let handle = OscHandle::new();

        let port1 = handle.client().await.unwrap().local_port();
        let port2 = handle.client().await.unwrap().local_port();
        assert_eq!(port1, port2);
    }

    /// Multiple `OscHandle`s each get their own port (no contention).
    #[tokio::test]
    async fn multiple_handles_get_distinct_ports() {
        let handle1 = OscHandle::new();
        let handle2 = OscHandle::new();

        let port1 = handle1.client().await.unwrap().local_port();
        let port2 = handle2.client().await.unwrap().local_port();
        assert_ne!(port1, port2);
    }

    /// `send()` succeeds through lazy init (UDP fire-and-forget).
    #[tokio::test]
    async fn handle_send_delegates_to_client() {
        let handle = OscHandle::new();
        // send is fire-and-forget over UDP; it should not error even without Ableton
        let result = handle.send("/live/test", vec![OscType::Int(1)]).await;
        assert!(result.is_ok());
    }

    /// Spin up a mock `AbletonOSC` server that replies to the sender's address
    /// (mirroring our `AbletonOSC` patch). Two `OscClient`s query it concurrently
    /// and each receives its own response — proving multi-instance works.
    #[tokio::test]
    async fn two_clients_receive_own_responses_from_mock_server() {
        use std::net::SocketAddr;

        // --- Mock AbletonOSC: listen on an ephemeral port, echo back to sender ---
        let mock = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let mock_port = mock.local_addr().unwrap().port();

        let mock_handle = tokio::spawn(async move {
            let mut buf = [0u8; 65536];
            // Handle exactly 2 queries then stop
            for _ in 0..2 {
                let (len, sender) = mock.recv_from(&mut buf).await.unwrap();
                // Decode the request
                let (_, packet) = decoder::decode_udp(&buf[..len]).unwrap();
                if let OscPacket::Message(msg) = packet {
                    // Build a response: echo the address back with the sender's port
                    // as payload so each client can verify it got *its own* response
                    let reply = OscPacket::Message(OscMessage {
                        addr: msg.addr,
                        args: vec![OscType::Int(i32::from(sender.port()))],
                    });
                    let bytes = encoder::encode(&reply).unwrap();
                    // Reply to the sender's actual address (the patched behavior)
                    mock.send_to(&bytes, sender).await.unwrap();
                }
            }
        });

        // --- Create two OscClients pointing at the mock server ---
        let ableton_addr: SocketAddr = format!("127.0.0.1:{mock_port}").parse().unwrap();

        let client_a = {
            let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
            let port = socket.local_addr().unwrap().port();
            (socket, port, ableton_addr)
        };
        let client_b = {
            let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
            let port = socket.local_addr().unwrap().port();
            (socket, port, ableton_addr)
        };

        assert_ne!(client_a.1, client_b.1, "clients must have different ports");

        // --- Both clients send a query ---
        let query = OscPacket::Message(OscMessage {
            addr: "/live/song/get/tempo".to_string(),
            args: vec![],
        });
        let query_bytes = encoder::encode(&query).unwrap();

        client_a
            .0
            .send_to(&query_bytes, ableton_addr)
            .await
            .unwrap();
        client_b
            .0
            .send_to(&query_bytes, ableton_addr)
            .await
            .unwrap();

        // --- Each client reads its own response ---
        let mut buf_a = [0u8; 65536];
        let mut buf_b = [0u8; 65536];

        let (len_a, _) =
            tokio::time::timeout(Duration::from_secs(1), client_a.0.recv_from(&mut buf_a))
                .await
                .expect("client A timed out")
                .unwrap();

        let (len_b, _) =
            tokio::time::timeout(Duration::from_secs(1), client_b.0.recv_from(&mut buf_b))
                .await
                .expect("client B timed out")
                .unwrap();

        // --- Verify each response contains that client's own port ---
        let (_, pkt_a) = decoder::decode_udp(&buf_a[..len_a]).unwrap();
        let (_, pkt_b) = decoder::decode_udp(&buf_b[..len_b]).unwrap();

        let port_in_a = match pkt_a {
            OscPacket::Message(m) => match m.args.first() {
                Some(OscType::Int(p)) => *p as u16,
                _ => panic!("unexpected response format"),
            },
            OscPacket::Bundle(_) => panic!("expected message"),
        };
        let port_in_b = match pkt_b {
            OscPacket::Message(m) => match m.args.first() {
                Some(OscType::Int(p)) => *p as u16,
                _ => panic!("unexpected response format"),
            },
            OscPacket::Bundle(_) => panic!("expected message"),
        };

        // The mock echoed back the sender's port — each client should see its own
        assert_eq!(
            port_in_a, client_a.1,
            "client A got a response meant for someone else"
        );
        assert_eq!(
            port_in_b, client_b.1,
            "client B got a response meant for someone else"
        );

        mock_handle.await.unwrap();
    }
}
