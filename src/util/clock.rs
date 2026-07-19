use std::{
    sync::atomic::{AtomicU32, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

static CURRENT_SYSTEM_SECS: AtomicU32 = AtomicU32::new(0);

pub fn init() {
    tokio::spawn(update_secs());
}

async fn update_secs() {
    let duration = Duration::from_secs(1);
    loop {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as u32;
        CURRENT_SYSTEM_SECS.store(secs, Ordering::Relaxed);
        tokio::time::sleep(duration).await;
    }
}

pub fn get_current_secs() -> u32 {
    CURRENT_SYSTEM_SECS.load(Ordering::Relaxed)
}
