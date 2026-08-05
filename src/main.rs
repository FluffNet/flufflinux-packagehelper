mod i18n;

use std::env;
use std::ffi::{CString, OsStr};
use std::os::raw::{c_char, c_int};
use std::process::Command;

use i18n::PackageFormat;

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

fn package_format(mime_type: &str) -> Option<PackageFormat> {
    let mime_type = mime_type.to_ascii_lowercase();

    if mime_type.contains("deb") {
        Some(PackageFormat::Debian)
    } else if mime_type.contains("rpm") || mime_type.contains("redhat-package") {
        Some(PackageFormat::Rpm)
    } else {
        None
    }
}

fn c_string(value: &str) -> CString {
    CString::new(value.replace('\0', "\u{fffd}")).expect("NUL bytes were replaced")
}

fn main() {
    let Some(file_argument) = env::args_os().nth(1) else {
        return;
    };

    let Some(package_format) =
        package_format(&detected_mime_type(&file_argument).unwrap_or_default())
    else {
        return;
    };

    let locale = i18n::system_locale();
    let translation = i18n::translation(&locale);
    let file_name = file_argument.to_string_lossy();
    let message = translation.message(&file_name, package_format);

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
    use super::{PackageFormat, package_format};

    #[test]
    fn recognizes_debian_mime_types() {
        assert_eq!(
            package_format("application/vnd.debian.binary-package"),
            Some(PackageFormat::Debian)
        );
        assert_eq!(
            package_format("application/x-deb"),
            Some(PackageFormat::Debian)
        );
    }

    #[test]
    fn recognizes_rpm_mime_types() {
        assert_eq!(
            package_format("application/x-rpm"),
            Some(PackageFormat::Rpm)
        );
        assert_eq!(
            package_format("application/x-redhat-package-manager"),
            Some(PackageFormat::Rpm)
        );
    }

    #[test]
    fn does_not_use_a_filename_as_a_mime_type() {
        assert_eq!(package_format("text/plain"), None);
    }
}
