pub fn get_cwd() -> std::io::Result<std::path::PathBuf> {
    std::env::current_dir()
}