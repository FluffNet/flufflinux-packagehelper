mod i18n;

use std::env;
use std::ffi::{CString, OsStr};
use std::os::raw::{c_char, c_int};
use std::process::Command;

unsafe extern "C" {
    fn flufflinux_show_information_dialog(
        title: *const c_char,
        message: *const c_char,
        accept_button: *const c_char,
        right_to_left: bool,
    ) -> c_int;
}

fn detected_mime_type(path: &OsStr) -> Option<String> {
    let output = Command::new("file")
        .args(["--brief", "--mime-type", "--"])
        .arg(path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn is_debian_package(mime_type: &str) -> bool {
    mime_type.to_ascii_lowercase().contains("deb")
}

fn c_string(value: &str) -> CString {
    CString::new(value.replace('\0', "\u{fffd}")).expect("NUL bytes were replaced")
}

fn main() {
    let Some(file_argument) = env::args_os().nth(1) else {
        return;
    };

    if !is_debian_package(&detected_mime_type(&file_argument).unwrap_or_default()) {
        return;
    }

    let locale = i18n::system_locale();
    let translation = i18n::translation(&locale);
    let file_name = file_argument.to_string_lossy();
    let message = translation.message(&file_name);

    let title = c_string(translation.title);
    let message = c_string(&message);
    let accept_button = c_string(translation.accept);

    unsafe {
        flufflinux_show_information_dialog(
            title.as_ptr(),
            message.as_ptr(),
            accept_button.as_ptr(),
            translation.rtl,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::is_debian_package;

    #[test]
    fn recognizes_debian_mime_types() {
        assert!(is_debian_package("application/vnd.debian.binary-package"));
        assert!(is_debian_package("application/x-deb"));
    }

    #[test]
    fn does_not_use_a_filename_as_a_mime_type() {
        assert!(!is_debian_package("text/plain"));
    }
}
