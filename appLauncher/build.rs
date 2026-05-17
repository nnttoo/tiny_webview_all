use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn copy_file(nama_file: &str, folder_target: &Path) {
    let file_sumber = PathBuf::from(nama_file);

    if file_sumber.exists() {
        let file_tujuan = folder_target.join(nama_file); 
        fs::copy(&file_sumber, &file_tujuan)
            .unwrap_or_else(|_| panic!("Gagal menyalin file {}", nama_file)); 
        println!("cargo:rerun-if-changed={}", nama_file);
    } else {
        // Jika file tidak ada di root, beri peringatan kuning di terminal saat build
        println!("cargo:warning=File '{}' tidak ditemukan di root folder project!", nama_file);
    }
}

fn dapatkan_folder_target() -> PathBuf { 
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut folder = PathBuf::from(out_dir); 
    while folder.file_name().unwrap() != "target" {
        folder.pop();
    } 
    let profil_aktif = env::var("PROFILE").unwrap(); 
    folder.push(profil_aktif);

    folder // Mengembalikan path akhir (misal: target/release/)
}

fn main() {
    // Hanya jalankan skrip ini jika kita sedang menargetkan OS Windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("favicon.ico"); // <-- Arahkan ke file .ico kamu
        res.compile().unwrap();
    } 

    let folder_target = dapatkan_folder_target(); 
    copy_file("splash.png", &folder_target);
    copy_file("icon.png", &folder_target);
}