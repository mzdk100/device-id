# Device ID Management Module

This Rust module is used to manage the unique identifiers (UUID) of devices. It attempts to recover the device ID from the cache directory, and if it fails, it generates a new device ID and saves it to a file.
It does not rely on the device information API provided by the operating system, has no sensitive permissions, is easy to use, and is simple to build.
The device ID relies on file system storage, meaning the device ID is not permanently immutable.
Supported platforms: Windows, Linux, MacOS, iOS, Android, WASM, etc.

## Features

- Recover the device ID from the cache file.
- If recovery fails, generate a new device ID and save it to the file.

## Installation

Add this library to your project using Cargo:

```shell
cargo add device-id
```

## Usage

```rust
use device_id::get_device_id;
let device_id = get_device_id();
println!("Device ID: {}", device_id);
```

## Error Handling

The `get_device_id` function never returns any errors. If it cannot recover the device ID from the file, it will regenerate one using the UUID V4 algorithm.

## Notes

- If recovery from the file fails, an error message will be logged, and a new device ID will be generated.
- If saving the new device ID to the file also fails, an error message will be logged, and in this case, there will be no opportunity to use this device ID next time.

## License

This project is licensed under the MIT License. Please check the LICENSE file for more information.

## Contribution

Welcome to submit Pull Requests or report issues.
