use alloc::boxed::Box;
use alloc::vec::Vec;
use embedded_io_async::{ErrorType, Read, Write};
use embedded_tls::{Aes128GcmSha256, TlsConfig, TlsConnection, TlsContext, TlsError, UnsecureProvider};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

use super::tcp::TcpStream; 

struct TlsBuffers {
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,
}

pub struct TlsStream {
    connection: TlsConnection<'static, TcpStream, Aes128GcmSha256>,
    _buffers: Box<TlsBuffers>,
}

impl ErrorType for TlsStream {
    type Error = TlsError;
}

impl TlsStream {
    pub async fn connect(tcp_stream: TcpStream, host: &str) -> Result<Self, TlsError> {
        
        let mut buffers = Box::new(TlsBuffers {
            read_buf: alloc::vec![0; 16384],
            write_buf: alloc::vec![0; 16384],
        });

        let read_ref: &'static mut [u8] = unsafe { core::mem::transmute(&mut buffers.read_buf[..]) };
        let write_ref: &'static mut [u8] = unsafe { core::mem::transmute(&mut buffers.write_buf[..]) };

        let mut connection = TlsConnection::new(tcp_stream, read_ref, write_ref);

        let config = TlsConfig::new()
            .with_server_name(host);


        let mut seed = [0u8; 32];

        getrandom::getrandom(&mut seed).unwrap();

        let mut rng = ChaCha20Rng::from_seed(seed);

        let mut provider = UnsecureProvider::new::<Aes128GcmSha256>(rng);

        connection.open(TlsContext::new(&config, &mut provider)).await?;

        Ok(Self {
            connection,
            _buffers: buffers,
        })
    }
}

impl Read for TlsStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.connection.read(buf).await
    }
}

impl Write for TlsStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.connection.write(buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.connection.flush().await
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __sync_synchronize() {
}