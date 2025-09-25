use anyhow::Result;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn run_script(
  script: &String,
  data: &[u8],
) -> Result<String> {
  if let Ok(mut cmd) = Command::new(script)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
  {
    let cmd_stdin = cmd.stdin.as_mut().unwrap();
    cmd_stdin.write_all(data)?;
    let output = cmd.wait_with_output()?;
    Ok(String::from_utf8(output.stdout).unwrap())
  } else {
    // TODO: This is temporary.  Hoist this so it kills all
    // the processes, or has an ability to restart
    // or something when the script is fixed.
    println!("-------------------------------");
    panic!(
      "Could not run script: {}\nYou'll need to fix (maybe permissions?) it and restart\n\n-----------------------------------",
      &script
    );
  }
}
