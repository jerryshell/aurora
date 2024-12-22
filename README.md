# Aurora

Video frame interpolation using machine learning

## How to use

1. Download executable file from [releases](https://github.com/jerryshell/aurora/releases)

2. Edit `.env` file

3. Run `aurora-cli` or `aurora-cli.exe`

## Fix

### vulkan-1.dll is missing from your computer

[Install Vulkan Runtime](https://vulkan.lunarg.com/sdk/home)

### macOS cannot verify that this app is free from malware

```bash
xattr -dr com.apple.quarantine ffmpeg rife-ncnn-vulkan
```

## Credits

- [ffmpeg](https://ffmpeg.org)
- [rife-ncnn-vulkan](https://github.com/nihui/rife-ncnn-vulkan)

## License

[GNU Affero General Public License v3.0](LICENSE)
