#![allow(unused_parens)]

use std::env;
use std::process::Command;

use dialog::DialogBox;

fn dialog_box(file: String) {
  let msg: String = format!("The file \"{}\" is a Debian package.
Debian packages aren’t compatible with Fluff Linux because Fluff Linux is Arch-based and uses a different packaging system.

To install software on Fluff Linux, please use the Software Center (Discover) through the application launcher or by using pacman in a terminal (e.g., Konsole)", file);
	dialog::Message::new(msg)
    .title("Fluff Linux Package Helper")
    .show()
    .expect("Could not display dialog box"); 
}

fn main() {
  let files: Vec<String> = env::args().collect();

  let file = Command::new("file")
    .arg("--mime-type")
    .arg(&files[1])
    .output()
    .expect("failed to execute file command");
  
  let file_out = String::from_utf8_lossy(&file.stdout);
  if(file_out.contains("deb"))
  {
    dialog_box(files[1].clone());
  } 
  
}
