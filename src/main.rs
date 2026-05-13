use bootloader::UefiBoot;
use std::path::Path;

fn main() {
    // 階層が変わったのでパスを調整
    let kernel_path = Path::new("../Rust_os/target/x86_64-unknown-none/debug/rust_os");
    let out_path = Path::new("../Rust_os/target/x86_64-unknown-none/debug/rust_os_uefi.img");
    
    if !kernel_path.exists() {
        panic!("Kernel binary not found at {:?}. Run 'cargo build' in Rust_os directory first.", kernel_path);
    }

    let mut boot = UefiBoot::new(kernel_path);
    boot.create_disk_image(out_path).expect("Failed to create UEFI disk image");
    
    println!("UEFI disk image created at: {:?}", out_path);
}
