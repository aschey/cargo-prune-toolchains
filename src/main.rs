use std::{
    error::Error,
    process::{Command, Stdio},
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let current_version = get_current_version()?;
    let toolchains = list_toolchains()?;

    let nightly_re = Regex::new(r"nightly-\d{4}-\d{2}-\d{2}").unwrap();
    let stable_re = Regex::new(r"\d\.\d+(\.\d+)?-\w+-\w+-\w+").unwrap();

    let mut remove_count = 0;
    for toolchain in toolchains {
        let toolchain = toolchain.trim();
        if nightly_re.is_match(toolchain)
            || (stable_re.is_match(toolchain) && !toolchain.starts_with(&current_version))
        {
            uninstall(toolchain)?;
            remove_count += 1;
        }
    }

    println!("Removed {remove_count} toolchains\n");
    println!("Remaining toolchains:");
    Command::new("rustup")
        .args(["toolchain", "list"])
        .spawn()?
        .wait()?;

    Ok(())
}

fn get_current_version() -> Result<String, Box<dyn Error>> {
    let output = Command::new("rustup")
        .args(["show"])
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    let out_str = String::from_utf8(output.stdout)?;
    let re = Regex::new(r"rustc\s(\d\.\d+)\.\d+")?;
    let group = re
        .captures(&out_str)
        .unwrap()
        .get(1)
        .ok_or("current version not found")?;
    Ok(group.as_str().to_string())
}

fn list_toolchains() -> Result<Vec<String>, Box<dyn Error>> {
    let output = Command::new("rustup")
        .args(["toolchain", "list"])
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    let toolchains = String::from_utf8(output.stdout)?;
    let toolchains = toolchains.split("\n").map(Into::into).collect();
    Ok(toolchains)
}

fn uninstall(arg: &str) -> Result<(), Box<dyn Error>> {
    Command::new("rustup")
        .args(["toolchain", "uninstall", arg])
        .spawn()?
        .wait_with_output()?;
    Ok(())
}
