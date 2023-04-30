use notify_rust::Notification;
use tunet_helper::{NetFlux, Result};

pub fn succeeded(flux: NetFlux) -> Result<()> {
    #[cfg(target_os = "macos")]
    let _ = notify_rust::set_application("com.berrysoft.tunet");
    let mut notify = Notification::new();
    #[cfg(target_os = "windows")]
    notify.app_id("com.berrysoft.tunet");
    notify
        .summary(&format!("登录成功：{}", flux.username))
        .body(&format!("流量：{}\n余额：{}", flux.flux, flux.balance))
        .show()?;
    Ok(())
}
