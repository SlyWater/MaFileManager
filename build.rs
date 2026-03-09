fn main() {
    if cfg!(target_os = "windows") {
        winres::WindowsResource::new()
            .set_icon("icon.ico")
            .set("LegalCopyright", "Copyright © 2026")
            .set("ProductName", "MaFileManager")
            .set("FileDescription", "MaFileManager")
            .compile()
            .unwrap();
    }
}
