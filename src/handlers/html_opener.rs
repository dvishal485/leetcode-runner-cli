use std::process::{Command, Stdio};
pub(crate) fn open_html(file_path: &str) {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd")
            .args(&["/C", &format!("start {}", file_path)])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    } else if cfg!(target_os = "macos") {
        let _ = Command::new("open")
            .arg(file_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    } else if cfg!(target_os = "linux") {
        let _ = Command::new("xdg-open")
            .arg(file_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    } else {
        eprintln!("HTML Renderer is not supported on this platform.");
    };
}
