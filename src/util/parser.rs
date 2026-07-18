use std::io::{Error, ErrorKind};

use tokio::io::{self, AsyncBufRead, AsyncReadExt};

use crate::{
    cluster::client::ClusterDTO,
    commands::{command_req::CommandType, command_res::ResType},
};

#[derive(Debug, Clone)]
pub enum ArrType {
    Txt = 23,
    I8 = 1,
    I16 = 2,
    I32 = 4,
    I64 = 5,
    U8 = 11,
    U16 = 12,
    U32 = 14,
    U64 = 15,
}

impl From<ArrType> for u8 {
    fn from(value: ArrType) -> Self {
        value as u8
    }
}

pub async fn read_small_string<R>(reader: &mut R) -> io::Result<String>
where
    R: AsyncBufRead + Unpin,
{
    let char = reader.read_u8().await?; //read first char '+'

    if char != b'+' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected String prefix + ",
        ));
    }

    let len = reader.read_u8().await?; //read len 'u8'

    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf).await?; //read till len chars

    String::from_utf8(buf)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "error reading small string"))
}

pub async fn read_bulk_string<R>(reader: &mut R) -> io::Result<String>
where
    R: AsyncBufRead + Unpin,
{
    let char = reader.read_u8().await?; //read first char '$'

    if char != b'$' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected String prefix $ ",
        ));
    }

    let len = reader.read_u32().await?; //read len 'u32'

    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf).await?; //read till len chars
    String::from_utf8(buf)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "error reading bulk string"))
}

pub async fn read_u64<R>(reader: &mut R) -> io::Result<u64>
where
    R: AsyncBufRead + Unpin,
{
    let num_type = reader.read_u8().await?;
    if num_type != b'@' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u64 prefix @ ",
        ));
    }
    let s = reader.read_u8().await?;
    if s != 15 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u64 prefix @15 ",
        ));
    }

    let value = reader.read_u64().await?;

    Ok(value)
}

pub async fn read_i64<R>(reader: &mut R) -> io::Result<i64>
where
    R: AsyncBufRead + Unpin,
{
    let num_type = reader.read_u8().await?;
    if num_type != b'@' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i64 prefix @ ",
        ));
    }
    let s = reader.read_u8().await?;
    if s != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i64 prefix @5 ",
        ));
    }

    let value = reader.read_i64().await?;

    Ok(value)
}

pub async fn read_u32<R>(reader: &mut R) -> io::Result<u32>
where
    R: AsyncBufRead + Unpin,
{
    let num_type = reader.read_u8().await?;
    if num_type != b'@' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u32 prefix @ ",
        ));
    }
    let s = reader.read_u8().await?;
    if s != 14 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u32 prefix @14 ",
        ));
    }

    let value = reader.read_u32().await?;

    Ok(value)
}

pub async fn read_i32<R>(reader: &mut R) -> io::Result<i32>
where
    R: AsyncBufRead + Unpin,
{
    let num_type = reader.read_u8().await?;
    if num_type != b'@' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i32 prefix @ ",
        ));
    }
    let s = reader.read_u8().await?;
    if s != 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i32 prefix @4 ",
        ));
    }

    let value = reader.read_i32().await?;

    Ok(value)
}

pub async fn read_u16<R>(reader: &mut R) -> io::Result<u16>
where
    R: AsyncBufRead + Unpin,
{
    let num_type = reader.read_u8().await?;
    if num_type != b'@' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u16 prefix @ ",
        ));
    }
    let s = reader.read_u8().await?;
    if s != 12 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u16 prefix @12 ",
        ));
    }

    let value = reader.read_u16().await?;

    Ok(value)
}

pub async fn read_i16<R>(reader: &mut R) -> io::Result<i16>
where
    R: AsyncBufRead + Unpin,
{
    let num_type = reader.read_u8().await?;
    if num_type != b'@' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i16 prefix @ ",
        ));
    }
    let s = reader.read_u8().await?;
    if s != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i16 prefix @2 ",
        ));
    }

    let value = reader.read_i16().await?;

    Ok(value)
}

pub async fn read_u8<R>(reader: &mut R) -> io::Result<u8>
where
    R: AsyncBufRead + Unpin,
{
    let num_type = reader.read_u8().await?;
    if num_type != b'@' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix @ ",
        ));
    }
    let s = reader.read_u8().await?;
    if s != 11 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix @11 ",
        ));
    }

    let value = reader.read_u8().await?;

    Ok(value)
}

pub async fn read_res_header<R>(reader: &mut R) -> io::Result<(ResType, Option<ClusterDTO>)>
where
    R: AsyncBufRead + Unpin,
{
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf).await?;
    if buf[0] != b'\n' || buf[1] != b'\n' {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "command should starts with '\n\n'",
        ));
    }
    let version = reader.read_u8().await?;
    if version != 1 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "unsupported protocol version",
        ));
    }

    let has_cluster_info = reader.read_u8().await?;
    let mut cluster: Option<ClusterDTO> = None;
    if has_cluster_info != 0 {
        let cluster_str = read_bulk_string(reader).await?;
        cluster = Some(
            serde_json::from_str(&cluster_str)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?,
        );
    }

    let res_type = reader.read_u8().await?;
    let res_type = ResType::try_from(res_type).unwrap();

    Ok((res_type, cluster))
}

pub fn append_bulk_string(buf: &mut Vec<u8>, data: &str) {
    buf.push(b'$');
    buf.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buf.extend_from_slice(data.as_bytes());
}

pub fn append_small_string(buf: &mut Vec<u8>, data: &str) {
    let bytes = data.as_bytes();
    assert!(
        bytes.len() <= u8::MAX as usize,
        "append_small_string: {} byte string exceeds the 255 byte small-string limit",
        bytes.len()
    );
    buf.push(b'+');
    buf.extend_from_slice(&(bytes.len() as u8).to_be_bytes());
    buf.extend_from_slice(bytes);
}

pub async fn read_i16_array<R>(reader: &mut R) -> io::Result<Vec<i16>>
where
    R: AsyncBufRead + Unpin,
{
    let arr_byte = reader.read_u8().await?;
    if arr_byte != b'*' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix * ",
        ));
    }
    let len = reader.read_u32().await?;

    let arr_type = reader.read_u8().await?;
    if arr_type != u8::from(ArrType::I16) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i16 prefix 2 ",
        ));
    }
    let mut arr = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let value = reader.read_i16().await?;
        arr.push(value);
    }

    Ok(arr)
}

pub async fn read_i32_array<R>(reader: &mut R) -> io::Result<Vec<i32>>
where
    R: AsyncBufRead + Unpin,
{
    let arr_byte = reader.read_u8().await?;
    if arr_byte != b'*' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix * ",
        ));
    }
    let len = reader.read_u32().await?;

    let arr_type = reader.read_u8().await?;
    if arr_type != u8::from(ArrType::I32) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i32 prefix 4 ",
        ));
    }
    let mut arr = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let value = reader.read_i32().await?;
        arr.push(value);
    }

    Ok(arr)
}

pub async fn read_i64_array<R>(reader: &mut R) -> io::Result<Vec<i64>>
where
    R: AsyncBufRead + Unpin,
{
    let arr_byte = reader.read_u8().await?;
    if arr_byte != b'*' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix * ",
        ));
    }
    let len = reader.read_u32().await?;

    let arr_type = reader.read_u8().await?;
    if arr_type != u8::from(ArrType::I64) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected i64 prefix 5 ",
        ));
    }
    let mut arr = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let value = reader.read_i64().await?;
        arr.push(value);
    }

    Ok(arr)
}

pub async fn read_u8_array<R>(reader: &mut R) -> io::Result<Vec<u8>>
where
    R: AsyncBufRead + Unpin,
{
    let arr_byte = reader.read_u8().await?;
    if arr_byte != b'*' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix * ",
        ));
    }
    let len = reader.read_u32().await?;

    let arr_type = reader.read_u8().await?;
    if arr_type != u8::from(ArrType::U8) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix 11 ",
        ));
    }
    let mut arr = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let value = reader.read_u8().await?;
        arr.push(value);
    }

    Ok(arr)
}

pub async fn read_u16_array<R>(reader: &mut R) -> io::Result<Vec<u16>>
where
    R: AsyncBufRead + Unpin,
{
    let arr_byte = reader.read_u8().await?;
    if arr_byte != b'*' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix * ",
        ));
    }
    let len = reader.read_u32().await?;

    let num_type = reader.read_u8().await?;
    if num_type != u8::from(ArrType::U16) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u16 prefix 12 ",
        ));
    }
    let mut arr = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let value = reader.read_u16().await?;
        arr.push(value);
    }

    Ok(arr)
}

pub async fn read_u32_array<R>(reader: &mut R) -> io::Result<Vec<u32>>
where
    R: AsyncBufRead + Unpin,
{
    let arr_byte = reader.read_u8().await?;
    if arr_byte != b'*' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix * ",
        ));
    }
    let len = reader.read_u32().await?;

    let arr_type = reader.read_u8().await?;
    if arr_type != u8::from(ArrType::U32) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u32 prefix 14 ",
        ));
    }
    let mut arr = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let value = reader.read_u32().await?;
        arr.push(value);
    }

    Ok(arr)
}

pub async fn read_txt_array<R>(reader: &mut R) -> io::Result<Vec<String>>
where
    R: AsyncBufRead + Unpin,
{
    let arr_byte = reader.read_u8().await?;
    if arr_byte != b'*' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected u8 prefix * ",
        ));
    }
    let len = reader.read_u32().await?;

    let arr_type = reader.read_u8().await?;
    if arr_type != u8::from(ArrType::Txt) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected Txt prefix 23 ",
        ));
    }
    let mut arr = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let value = read_bulk_string(reader).await?;
        arr.push(value);
    }

    Ok(arr)
}

pub fn append_txt_array(buf: &mut Vec<u8>, data: &[String]) {
    buf.extend_from_slice(b"*");
    buf.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buf.extend_from_slice(&u8::from(ArrType::Txt).to_be_bytes());
    for d in data {
        append_bulk_string(buf, d);
    }
}

pub fn append_req_header(
    buf: &mut Vec<u8>,
    cmd_type: CommandType,
    client_uuid: uuid::Uuid,
    hash: u64,
    timestamp: i64,
) {
    buf.extend_from_slice(b"\n\n");
    let v = 1 as u8;
    buf.extend_from_slice(&v.to_be_bytes());
    buf.extend_from_slice(&v.to_be_bytes());
    buf.extend_from_slice(client_uuid.as_bytes());
    buf.extend_from_slice(&hash.to_be_bytes());
    buf.extend_from_slice(&timestamp.to_be_bytes());
    match cmd_type {
        CommandType::Cfg => {
            let one = 1 as u8;
            buf.extend_from_slice(&one.to_be_bytes());
        }
        CommandType::Data => {
            let two = 2 as u8;
            buf.extend_from_slice(&two.to_be_bytes());
        }

        CommandType::None => {
            let max = 255 as u8;
            buf.extend_from_slice(&max.to_be_bytes());
        }
    }
}

pub fn append_i8(buf: &mut Vec<u8>, val: i8) {
    buf.push(b'@');
    let u_8: u8 = 1;
    buf.extend_from_slice(&u_8.to_be_bytes());
    buf.extend_from_slice(&val.to_be_bytes());
}

pub fn append_i16(buf: &mut Vec<u8>, val: i16) {
    buf.push(b'@');
    let u_8: u8 = 2;
    buf.extend_from_slice(&u_8.to_be_bytes());
    buf.extend_from_slice(&val.to_be_bytes());
}

pub fn append_i32(buf: &mut Vec<u8>, val: i32) {
    buf.push(b'@');
    let u_8: u8 = 4;
    buf.extend_from_slice(&u_8.to_be_bytes());
    buf.extend_from_slice(&val.to_be_bytes());
}

pub fn append_i64(buf: &mut Vec<u8>, val: i64) {
    buf.push(b'@');
    let u_8: u8 = 5;
    buf.extend_from_slice(&u_8.to_be_bytes());
    buf.extend_from_slice(&val.to_be_bytes());
}

pub fn append_u8(buf: &mut Vec<u8>, val: u8) {
    buf.push(b'@');
    let u_8: u8 = 11;
    buf.extend_from_slice(&u_8.to_be_bytes());
    buf.extend_from_slice(&val.to_be_bytes());
}
pub fn append_u16(buf: &mut Vec<u8>, val: u16) {
    buf.push(b'@');
    let u_8: u8 = 12;
    buf.extend_from_slice(&u_8.to_be_bytes());
    buf.extend_from_slice(&val.to_be_bytes());
}
pub fn append_u32(buf: &mut Vec<u8>, val: u32) {
    buf.push(b'@');
    let u_8: u8 = 14;
    buf.extend_from_slice(&u_8.to_be_bytes());
    buf.extend_from_slice(&val.to_be_bytes());
}

pub fn append_u64(buf: &mut Vec<u8>, val: u64) {
    buf.push(b'@');
    let u_8: u8 = 15;
    buf.extend_from_slice(&u_8.to_be_bytes());
    buf.extend_from_slice(&val.to_be_bytes());
}
