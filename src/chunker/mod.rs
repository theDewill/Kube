use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB chunks


//Util Class-->
struct FileParser {
    current_node : Vec<u8>,

}

#[derive(Serialize, Deserialize)]
struct ChunkInfo {
    index: usize,
    hash: String,
}

#[derive(Serialize, Deserialize)]
struct FileInfo {
    original_name: String,
    file_type: FileType,
    chunks: Vec<ChunkInfo>,
    encryption_key: Vec<u8>,
    nonce: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
enum FileType {
    Doc,
    Image,
    Video,
    Audio,
}

pub fn file_distrib(file_path: &str, log_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_type = identify_file_type(file_path)?;
    let mut file_contents = read_file(file_path)?;

    // Step 1: Compress the file
    file_contents = compress_file(&file_contents, &file_type)?;
    println!("Compression Done..")

    // Step 2: Encrypt the file
    let (encrypted_contents, key, nonce) = encrypt_file(&file_contents)?;

    // Step 3: Chunk the file
    let chunks = chunk_file(&encrypted_contents)?;

    // Step 4: Record in log
    let file_info = create_file_info(file_path, file_type, &chunks, &key, &nonce);
    record_log(log_path, &file_info)?;

    Ok(())
}

fn identify_file_type(file_path: &str) -> Result<FileType, Box<dyn std::error::Error>> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or("Failed to get file extension")?;

    match extension.to_lowercase().as_str() {
        "doc" | "docx" | "pdf" | "txt" => Ok(FileType::Doc),
        "jpg" | "jpeg" | "png" | "gif" => Ok(FileType::Image),
        "mp4" | "avi" | "mov" => Ok(FileType::Video),
        "mp3" | "wav" | "ogg" => Ok(FileType::Audio),
        _ => Err("Unsupported file type".into()),
    }
}

fn read_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn compress_file(contents: &[u8], file_type: &FileType) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // This is a placeholder. In a real implementation, you'd use different compression
    // algorithms based on the file type.
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(contents)?;
    Ok(encoder.finish()?)
}

fn encrypt_file(contents: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();
    let nonce: [u8; 12] = rng.gen();

    let cipher = Aes256Gcm::new(Key::from_slice(&key));
    let encrypted = cipher
        .encrypt(Nonce::from_slice(&nonce), contents.as_ref())
        .map_err(|e| format!("Encryption failed: {:?}", e))?;

    Ok((encrypted, key.to_vec(), nonce.to_vec()))
}

fn chunk_file(contents: &[u8]) -> Result<Vec<ChunkInfo>, Box<dyn std::error::Error>> {
    let chunk_count = (contents.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;
    let chunks = Arc::new(Mutex::new(Vec::with_capacity(chunk_count)));

    thread::scope(|s| {
        for (index, chunk) in contents.chunks(CHUNK_SIZE).enumerate() {
            let chunks = Arc::clone(&chunks);
            s.spawn(move || {
                let mut hasher = Sha256::new();
                hasher.update(chunk);
                let hash = format!("{:x}", hasher.finalize());
                let chunk_info = ChunkInfo { index, hash };
                chunks.lock().unwrap().push(chunk_info);
            });
        }
    });

    Ok(Arc::try_unwrap(chunks).unwrap().into_inner().unwrap())
}

fn create_file_info(
    file_path: &str,
    file_type: FileType,
    chunks: &[ChunkInfo],
    key: &[u8],
    nonce: &[u8],
) -> FileInfo {
    FileInfo {
        original_name: Path::new(file_path).file_name().unwrap().to_str().unwrap().to_string(),
        file_type,
        chunks: chunks.to_vec(),
        encryption_key: key.to_vec(),
        nonce: nonce.to_vec(),
    }
}

fn record_log(log_path: &str, file_info: &FileInfo) -> Result<(), Box<dyn std::error::Error>> {
    let log_entry = serde_json::to_string(file_info)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    writeln!(file, "{}", log_entry)?;
    Ok(())
}
