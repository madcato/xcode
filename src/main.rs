use clap::Parser;
use std::env;
use subprocess::Exec;

/// Struct for command line arguments
#[derive(Parser)]
struct Args {
    /// The scheme to use for building the project
    #[clap(short, long)]
    scheme: String,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    let scheme = args.scheme;

    // Change project directory
    let current_dir = env::current_dir().expect("Error reading current dir");
    println!("Current dir: {:?}", current_dir);

    let project_name = match find_project_name(&current_dir) {
        Some(name) => name,
        None => {
            eprintln!("Project name not found");
            std::process::exit(1);
        }
    };
    println!("Project name: {}", project_name);

    // Build and archive application using xcodebuild
    build_and_archive(&project_name, &scheme);

    // Upload archive to TestFlight using altool
    upload_to_testflight(&project_name);
}

fn find_project_name(dir: &std::path::Path) -> Option<String> {
    let xcodeproj_path = dir.read_dir().expect("Error reading directory")
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("xcodeproj"))
        .map(|entry| entry.file_name().into_string().expect("Error reading archive name"));
    
    xcodeproj_path
}

fn build_and_archive(project_name: &str, scheme: &str) {
    let status = Exec::cmd("xcodebuild")
        .args(&[
            "-scheme", scheme,
            "-archivePath", &format!("build/{}.xcarchive", project_name),
            "archive"
        ])
        .join()
        .expect("Error executing xcodebuild archive");
    if !status.success() {
        eprintln!("Error: xcodebuild archive fail");
        std::process::exit(1);
    }
}

fn upload_to_testflight(project_name: &str) {
    let status = Exec::cmd("xcrun")
        .args(&[
            "altool",
            "--upload-app",
            "-f", &format!("build/{}.xcarchive", project_name),
            "-t", "ios",
            "--apiKey", "<API_KEY>",
            "--apiIssuer", "<API_ISSUER>"
        ])
        .join()
        .expect("Error ejecutando altool");
    if !status.success() {
        eprintln!("Error: altool upload falló");
        std::process::exit(1);
    }
}
