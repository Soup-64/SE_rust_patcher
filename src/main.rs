use std::path::Path;
use std::process::Command;

fn main() {
    println!("Starting patcher...");
    println!("please make sure you have proton 5.0 and 6.3 installed!!!");

    println!("checking for protontricks...");

    if check_pt() {
        println!("protontricks detected");
    } else {
        println!("protontricks was not detected! Aborting...");
        std::process::exit(1);
    }

    println!("checking for space enginners...");

    if check_se() {
        println!("Space Engineers detected");
    } else {
        println!("Space Engineers was not detected! Aborting...");
        std::process::exit(1);
    }

    println!("all requirements were found, continuing with patching process");
    println!("running SE with proton 5.0-10...");

    run_se("5.0");

    println!("files generated, running protontricks (this will take a while)");

    run_pt();

    println!("Patcher completed, space engineers will run now, but stability is not guaranteed");
    println!("remember to set proton version to latest, and protonGE is recomended.");
}

fn run_se(version: &str) {
    Command::new("./launch.sh")
        .arg(&version)
        .output()
        .expect("SE could not be run");
}

fn run_pt() {
    Command::new("protontricks")
        .args(&["244850", "-q", "-f", "dotnet48"])
        .output()
        .expect("protontricks could not be run");
}

fn check_pt() -> bool {
    let pt_exists = Command::new("which")
        //using which instead of a directory test to confirm
        //that it is installed and actually works
        .arg(&"protontricks")
        .output()
        .expect("which failed to execute"); //if this happens the distro is probably fucked
    return pt_exists.status.success();
}

fn check_se() -> bool {
    let home_dir = home::home_dir()
        .expect("You don't have a home directory")
        .to_string_lossy()
        .into_owned();
    let se_dir = format!(
        "{}/.local/share/Steam/steamapps/common/SpaceEngineers/",
        home_dir
    );

    return Path::new(&se_dir).is_dir();
}