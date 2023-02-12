# Aurora

Video **Frame Interpolation** using machine learning.

Free and **TRUE** open source.

## Prerequired

* Download [FFmpeg](https://ffmpeg.org/download.html) and add it to the `PATH` environment variable.
* Download [rife-ncnn-vulkan release](https://github.com/nihui/rife-ncnn-vulkan/releases) according to your system, extract it, rename it to `rife-ncnn-vulkan`, and put it in the root directory of the project.

## Run

```bash
VIDEO_GLOB=*.mp4 FRAME_MULTIPLE=2 aurora-cli
```

## Fix: macOS cannot verify that this app is free from malware

```bash
xattr -dr com.apple.quarantine rife-ncnn-vulkan
```

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0)
