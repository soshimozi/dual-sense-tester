use hidapi::HidApi;
use std::fs::OpenOptions;
use std::io::Write;

pub fn find_dualsense_path() -> Option<String> {
    let api = HidApi::new().ok()?;
    let vid = 0x054c;
    let pid = 0x0ce6;

    api.device_list()
        .find(|d| d.vendor_id() == vid && d.product_id() == pid)
        .map(|d| d.path().to_string_lossy().into_owned())
}

pub fn write_report(path: &str, report: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    file.write_all(report)?;
    Ok(())
}
