use interprocess::local_socket::{
    GenericNamespaced,  ToNsName, 
    tokio::Stream, 
    traits::tokio::{  Stream as TokioStreamTrait}, // Import the trait with an alias to avoid conflict
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io; 
pub async fn send_to_ipc(ipc_path: &str, data: &[u8]) -> io::Result<Vec<u8>> { 
    let Ok(server_name) = ipc_path.to_string().to_ns_name::<GenericNamespaced>() else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid IPC path"));
    };
     
    let Ok(mut stream) = Stream::connect(server_name).await else {
        return Err(io::Error::new(io::ErrorKind::NotConnected, "Failed to connect to IPC server"));
    }; 
 
    let payload_len = data.len() as u32;
    let len_header = payload_len.to_be_bytes(); // Generates [u8; 4]
    
    let Ok(_) = stream.write_all(&len_header).await else {
        return Err(io::Error::new(io::ErrorKind::WriteZero, "Failed to write length header"));
    };
 
    let Ok(_) = stream.write_all(data).await else {
        return Err(io::Error::new(io::ErrorKind::WriteZero, "Failed to write payload"));
    };
    
    _ = stream.flush().await;
 
    let mut resp_len_header = [0u8; 4];
    let Ok(_) = stream.read_exact(&mut resp_len_header).await else {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Failed to read response header"));
    };
    let expected_response_length = u32::from_be_bytes(resp_len_header) as usize;

    let mut response_buffer = vec![0u8; expected_response_length];
    
    let Ok(_) = stream.read_exact(&mut response_buffer).await else {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Failed to read full response payload"));
    };

    Ok(response_buffer)
}