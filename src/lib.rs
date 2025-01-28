//! 设备ID管理模块

use cache_dir::{get_cache_dir, NoSuchDirectoryError};
use std::{
    fs::File,
    io::{Error as IoError, Read, Write},
    sync::LazyLock,
};
use tracing::error;
use uuid::Error as UuidError;
pub use uuid::Uuid;

/// 定义可能出现的错误类型
#[allow(dead_code)]
#[derive(Debug)]
enum DeviceIdError {
    /// 缓存目录不存在
    NoCacheDir(NoSuchDirectoryError),
    /// 文件打开失败
    NoFile(IoError),
    /// 数据无效
    DataInvalid(UuidError),
}

/// 常量，存储UUID的文件名
const ID_FILE_NAME: &str = "device.id";

fn try_get_device_id() -> Result<Uuid, DeviceIdError> {
    let path = get_cache_dir()
        .map_err(DeviceIdError::NoCacheDir)?
        .join(ID_FILE_NAME);
    let mut file = File::open(path).map_err(DeviceIdError::NoFile)?;
    let mut buf = Vec::with_capacity(16);
    file.read_to_end(&mut buf).map_err(DeviceIdError::NoFile)?;
    Ok(Uuid::from_slice(&buf).map_err(DeviceIdError::DataInvalid)?)
}

static ID_FILE: LazyLock<Uuid> = LazyLock::new(|| {
    try_get_device_id().map_or_else(
        |e| {
            error!(
                ?e,
                "Can't restore the device id from file. Using new device id."
            );
            let new = Uuid::new_v4();
            const ERROR_MSG: &str =
                "Can't save device id to file, it will not be restored next time.";
            match get_cache_dir() {
                Ok(path) => match File::create(path.join(ID_FILE_NAME)) {
                    Ok(mut file) => match file.write_all(new.as_bytes()) {
                        Ok(_) => (),
                        Err(e) => error!(?e, ERROR_MSG),
                    },
                    Err(e) => error!(?e, ERROR_MSG),
                },
                Err(e) => error!(?e, ERROR_MSG),
            }
            new
        },
        |i| i,
    )
});

/// 获取设备ID的函数。
///
/// 尝试从文件中恢复设备ID，如果失败则生成一个新的设备ID并保存到文件中。
///
/// # 返回值
///
/// 返回一个 `Uuid` 类型的设备ID。
///
/// # 错误处理
///
/// 如果从文件中恢复设备ID失败，会记录错误信息并生成一个新的设备ID。
/// 如果保存新的设备ID到文件时也失败，会记录错误信息，在这种情况下下次将没有机会使用这个设备ID。
///
/// # 示例
///
/// ```rust
/// use device_id::get_device_id;
/// let device_id = get_device_id();
/// println!("Device ID: {}", device_id);
/// ```
pub fn get_device_id() -> Uuid {
    ID_FILE.clone()
}
