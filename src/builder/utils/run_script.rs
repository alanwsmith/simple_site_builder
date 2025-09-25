use anyhow::Result;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

pub fn run_script(
  script: &String,
  data: &[u8],
) -> Result<String> {
  // let file = fs::File::open(script)?;
  // let bit_update = 0o700;
  // let mut permissions = file.metadata()?.permissions();
  // permissions.set_mode(permissions.mode() | bit_update);
  // file.set_permissions(permissions)?;

  let mut cmd = Command::new(script)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
  let cmd_stdin = cmd.stdin.as_mut().unwrap();
  cmd_stdin.write_all(data)?;
  let output = cmd.wait_with_output()?;
  Ok(String::from_utf8(output.stdout).unwrap())
}
