fn main() {
    dotenv::dotenv().ok();

    let video_glob = std::env::var("VIDEO_GLOB").expect("VICEO_GLOB must be set");
    println!("video_glob: {video_glob}");

    let path_list = glob::glob(&video_glob)
        .expect("failed to read glob pattern")
        .filter_map(|e| e.ok())
        .map(|path| path.to_str().unwrap_or("").to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    println!("path_list: {path_list:?}");

    for video_filename in path_list {
        println!("video_filename: {video_filename}");

        let frame_multiple = match std::env::var("FRAME_MULTIPLE") {
            Ok(str) => str.parse::<usize>().unwrap_or(2),
            Err(_) => 2,
        };
        println!("frame_multiple: {frame_multiple}");

        aurora_cli::run(&video_filename, frame_multiple);
    }
}
