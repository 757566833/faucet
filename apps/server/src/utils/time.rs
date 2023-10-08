use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub async fn get_current_time() -> Result<u64, SystemTimeError> {
    // 获取当前时间
    let current_time = SystemTime::now();

    // 计算当前时间与UNIX纪元（1970-01-01T00:00:00Z）的时间间隔
    let duration_since_epoch_result = current_time.duration_since(UNIX_EPOCH);
    match duration_since_epoch_result {
        Ok(duration_since_epoch) => {
            let unix_timestamp = duration_since_epoch.as_secs();
            return Ok(unix_timestamp);
        }
        Err(e) => return Err(e),
    }
}
