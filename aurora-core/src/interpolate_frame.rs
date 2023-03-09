pub fn interpolate_frame(
    target_frame_count: usize,
    video_frames_dir_name: &str,
    video_interpolate_frames_dir_name: &str,
    j: &str,
) {
    let interpolate_frame_cmd_str = if cfg!(target_os = "windows") {
        format!(
            r"rife-ncnn-vulkan\rife-ncnn-vulkan.exe -m rife-v4.6 -j {j} -n {target_frame_count} -i {video_frames_dir_name} -o {video_interpolate_frames_dir_name}"
        )
    } else {
        format!(
            r"rife-ncnn-vulkan/rife-ncnn-vulkan -m rife-v4.6 -j {j} -n {target_frame_count} -i {video_frames_dir_name} -o {video_interpolate_frames_dir_name}"
        )
    };
    tracing::info!("interpolate_frame_cmd_str: {interpolate_frame_cmd_str}");

    let interpolate_frame_cmd_output = crate::execute_cmd(&interpolate_frame_cmd_str);
    tracing::info!("interpolate_frame_cmd_output: {interpolate_frame_cmd_output:?}");
}
