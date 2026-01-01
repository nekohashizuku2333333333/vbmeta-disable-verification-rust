use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::process;

// Magic for the vbmeta image header.
const AVB_MAGIC: &[u8; 4] = b"AVB0";
const AVB_MAGIC_LEN: usize = 4;

// Information about the verification flags
const FLAGS_OFFSET: u64 = 123;
const FLAG_DISABLE_VERITY: u8 = 0x01;
const FLAG_DISABLE_VERIFICATION: u8 = 0x02;

fn help() {
    println!("vbmeta-disable-verification In Rust \nciallo (∠·ω )⌒★");
    println!("Originally Powered by LibXZR <i@xzr.moe> , Remaked by zjw2017 <1987836456@qq.com>,Rewrite By Rust.");
    println!("Usage:\n vbmeta-disable-verification [options] <vbmeta-image>");
    println!("Options:");
    println!("  --disable-verity        Disable dm-verity");
    println!("  --disable-verification  Disable dm-verification");
    println!("If no options are provided, both flags will be set by default.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut filename: Option<String> = None;
    let mut disable_verity = 0u8;
    let mut disable_verification = 0u8;

    // 解析命令行参数
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--disable-verity" => {
                disable_verity = 1;
            }
            "--disable-verification" => {
                disable_verification = 1;
            }
            "--help" => {
                help();
                return;
            }
            other => {
                // 非选项参数视为文件名
                if filename.is_some() {
                    println!("Error: Only one vbmeta image can be specified.");
                    help();
                    process::exit(1);
                }
                filename = Some(other.to_string());
            }
        }
        i += 1;
    }

    // 如果没有指定任何选项，默认禁用所有验证
    if disable_verity == 0 && disable_verification == 0 {
        disable_verity = 1;
        disable_verification = 1;
    }

    // 构建标志位掩码
    let mut flags: u8 = 0;
    if disable_verity == 1 {
        flags |= FLAG_DISABLE_VERITY;
    }
    if disable_verification == 1 {
        flags |= FLAG_DISABLE_VERIFICATION;
    }

    let filename = match filename {
        Some(f) => f,
        None => {
            help();
            return;
        }
    };

    let mut file = match OpenOptions::new().read(true).write(true).open(&filename) {
        Ok(f) => f,
        Err(_) => {
            println!("Error: Unable to access '{}'.", filename);
            process::exit(1);
        }
    };

    // 读取并检查 magic
    let mut magic = [0u8; AVB_MAGIC_LEN];
    if let Err(_) = file.read_exact(&mut magic) {
        println!("Error: '{}' is not a valid vbmeta image.", filename);
        process::exit(1);
    }
    if magic != *AVB_MAGIC {
        println!("Error: '{}' is not a valid vbmeta image.", filename);
        process::exit(1);
    }

    // 读取当前 flags 字节
    if let Err(_) = file.seek(SeekFrom::Start(FLAGS_OFFSET)) {
        println!("Error: Failed when patching the vbmeta image");
        process::exit(1);
    }

    let mut buf = [0u8; 1];
    if let Err(_) = file.read_exact(&mut buf) {
        println!("Error: Failed when patching the vbmeta image");
        process::exit(1);
    }

    buf[0] |= flags;

    // 写回修改后的 flags
    if let Err(_) = file.seek(SeekFrom::Start(FLAGS_OFFSET)) {
        println!("Error: Failed when patching the vbmeta image");
        process::exit(1);
    }
    if let Err(_) = file.write_all(&buf) {
        println!("Error: Failed when patching the vbmeta image");
        process::exit(1);
    }

    println!(
        "Successfully disabled verification on vbmeta image: {}.",
        filename
    );
    print!("Disabled flags: ");
    if disable_verity == 1 {
        print!("dm-verity ");
    }
    if disable_verification == 1 {
        print!("dm-verification");
    }
    println!();
}
