#![allow(non_snake_case)]

const QSIZE: usize = 8;
const QSIZE_DOUBLE: usize = QSIZE * 2;
fn main() {
    let dataBuf = [510_u16; QSIZE];
    let mut sendBuf = [0_u8; QSIZE_DOUBLE];
    let mut j = 0;
    for (i, item) in dataBuf.iter().enumerate() {
        let bytes = item.to_be_bytes();
        j = i * 2;
        sendBuf[j] = bytes[0];
        sendBuf[j + 1] = bytes[1];
    }
    println!("send buffer({}): {:?}", sendBuf.len(), sendBuf);
}
