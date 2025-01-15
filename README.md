# 设备ID管理模块

这个Rust模块用于管理设备的唯一标识符（UUID）。它尝试从缓存目录中恢复设备ID，如果失败则生成一个新的设备ID并保存到文件中。
不依赖操作系统提供的设备信息API，没有任何敏感权限，易于使用，构建简单。
设备ID依赖于文件系统的保存，这意味着设备ID并不是永久不变的。
支持的平台： Windows, Linux, MacOS, iOS, android, WASM等。

## 功能

- 从缓存文件中恢复设备ID。
- 如果恢复失败，生成一个新的设备ID并保存到文件中。

## 安装

在您的项目中使用Cargo来添加这个库：

```shell
cargo add device-id
```

## 使用方法

```rust
use device_id::get_device_id;
let device_id = get_device_id();
println!("Device ID: {}", device_id);
```

## 错误处理

`get_device_id` 函数永远不会返回任何错误，如果无法从文件恢复设备ID则使用UUID V4算法重新生成。

## 注意事项

- 如果从文件中恢复设备ID失败，会记录错误信息并生成一个新的设备ID。
- 如果保存新的设备ID到文件时也失败，会记录错误信息，在这种情况下下次将没有机会使用这个设备ID。

## 许可证

本项目采用MIT许可证。请查看LICENSE文件了解更多信息。

## 贡献

欢迎提交Pull Request或报告问题。
