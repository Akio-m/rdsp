use std::{fs::File, env, io::Read, str};

extern crate wav_read;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args : {:?}", args);
    let case = &args[1]; // 起動するプログラムを分けるようにしたら使うかも
    println!("case: {}", case);
    let filename = &args[2]; // readするファイル
    println!("filename: {}", filename);

    let mut f = File::open(filename).expect("file not found.");

    let mut riffId: [u8; 4] = [0; 4];
    f.read_exact(&mut riffId);
    println!("riffId: {}", str::from_utf8(&riffId).unwrap());

    let mut fileSize: [u8; 4] = [0; 4];
    f.read_exact(&mut fileSize);
    println!("fileSize: {}", u32::from_ne_bytes(fileSize)); // from_be_bytesとの差は？

    let mut waveId: [u8; 4] = [0; 4];
    f.read_exact(&mut waveId);
    println!("waveId: {}", str::from_utf8(&waveId).unwrap());

    let mut fmtId: [u8; 4] = [0; 4];
    f.read_exact(&mut fmtId);
    println!("fmtId: {}", str::from_utf8(&fmtId).unwrap());

    let mut formatSize: [u8; 4] = [0; 4];
    f.read_exact(&mut formatSize);
    println!("formatSize: {}", u32::from_ne_bytes(formatSize));

    let mut formatId: [u8; 2] = [0; 2];
    f.read_exact(&mut formatId);
    println!("formatId: {}", u16::from_ne_bytes(formatId));

    let mut channelSize: [u8; 2] = [0; 2];
    f.read_exact(&mut channelSize);
    println!("channelSize: {}", u16::from_ne_bytes(channelSize));

    let mut sampleRate: [u8; 4] = [0; 4];
    f.read_exact(&mut sampleRate);
    println!("sampleRate: {}", u32::from_ne_bytes(sampleRate));

    let mut bytePerSed: [u8; 4] = [0; 4]; // 1秒間のデータのバイト数
    f.read_exact(&mut bytePerSed);
    println!("bytePerSed: {}", u32::from_ne_bytes(bytePerSed));

    let mut blockSize: [u8; 2] = [0; 2]; // BlockSize = Byte / sample * channelSize, 1データのバイト数
    f.read_exact(&mut blockSize);
    println!("blockSize: {}", u16::from_ne_bytes(blockSize));

    let mut quantizationBitSize: [u8; 2] = [0; 2]; // 量子化サイズ
    f.read_exact(&mut quantizationBitSize);
    println!("quantizationBitSize: {}", u16::from_ne_bytes(quantizationBitSize));

    let mut dataId: [u8; 4] = [0; 4];
    f.read_exact(&mut dataId);
    println!("dataId: {}", str::from_utf8(&dataId).unwrap());

    let mut dataSize: [u8; 4] = [0; 4];
    f.read_exact(&mut dataSize);
    println!("dataSize: {}", u32::from_ne_bytes(dataSize));

    // TODO: bodyを得る
}