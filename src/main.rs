mod i18n;

use std::env;
use std::ffi::{CString, OsStr};
use std::os::raw::{c_char, c_int};
use std::process::Command;

use i18n::PackageFormat;

#[cfg(not(test))]
unsafe extern "C" {
    fn flufflinux_show_information_dialog(
        title: *const c_char,
        message: *const c_char,
        accept_button: *const c_char,
        right_to_left: bool,
    ) -> c_int;
}

// Let logic tests run without loading Qt
#[cfg(test)]
unsafe fn flufflinux_show_information_dialog(
    _title: *const c_char,
    _message: *const c_char,
    _accept_button: *const c_char,
    _right_to_left: bool,
) -> c_int {
    0
}

// Ask file to identify content without using the file name
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

// Accept only the package formats registered by the desktop entry
fn package_format(mime_type: &str) -> Option<PackageFormat> {
    match mime_type.trim().to_ascii_lowercase().as_str() {
        "application/vnd.debian.binary-package"
        | "application/x-deb"
        | "application/x-debian-package"
        | "application/x-debian-packages" => Some(PackageFormat::Debian),
        "application/x-rpm" | "application/x-redhat-package-manager" => Some(PackageFormat::Rpm),
        _ => None,
    }
}

// Prepare safe text for the Qt bridge
fn c_string(value: &str) -> CString {
    CString::new(value.replace('\0', "\u{fffd}")).expect("NUL bytes were replaced")
}

fn main() {
    // Desktop file launches provide one selected file
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

    // Keep all strings alive until the modal dialog closes
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
        for mime_type in [
            "application/vnd.debian.binary-package",
            "application/x-deb",
            "application/x-debian-package",
            "application/x-debian-packages",
        ] {
            assert_eq!(package_format(mime_type), Some(PackageFormat::Debian));
        }
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
