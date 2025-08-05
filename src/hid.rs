use crate::error;
use hidapi::{self, HidApi, HidDevice};

const REPORT_ID: u8 = 1;
const MIN_BRIGHTNESS: u32 = 400;
const MAX_BRIGHTNESS: u32 = 60000;
const BRIGHTNESS_RANGE: u32 = MAX_BRIGHTNESS - MIN_BRIGHTNESS;
const SD_PRODUCT_ID: u16 = 0x1114;
const SD_VENDOR_ID: u16 = 0x05ac;
const SD_INTERFACE_NR: i32 = 0x7;
const BUFFER_CAPACITY: usize = 7;

fn set_brightness(
    handle: &mut hidapi::HidDevice,
    brightness: u32,
) -> Result<(), error::DisplayError> {
    let mut buf = Vec::with_capacity(7);

    buf.push(REPORT_ID);
    buf.extend(brightness.to_le_bytes());
    buf.extend(0_u16.to_le_bytes());
    handle.send_feature_report(&mut buf)?;

    Ok(())
}

pub fn set_brightness_percent(
    handle: &mut hidapi::HidDevice,
    brightness: u8,
) -> Result<(), error::DisplayError> {
    let nits =
        ((brightness as f32 * BRIGHTNESS_RANGE as f32) / 100.0 + MIN_BRIGHTNESS as f32) as u32;
    let nits = std::cmp::min(nits, MAX_BRIGHTNESS);
    let nits = std::cmp::max(nits, MIN_BRIGHTNESS);
    set_brightness(handle, nits)?;
    Ok(())
}

fn get_brightness(
    studio_display_handle: &mut hidapi::HidDevice,
) -> Result<u32, error::DisplayError> {
    let mut buf = Vec::with_capacity(BUFFER_CAPACITY);

    buf.push(REPORT_ID);
    buf.extend(0_u32.to_le_bytes());
    buf.extend(0_u16.to_le_bytes());
    let size = studio_display_handle.get_feature_report(&mut buf)?;

    if size != buf.len() {
        return Err(error::DisplayError::InvalidBitSize(
            error::InvalidBitSize::new(buf.len(), size),
        ));
    }

    let bytes_recieved = buf[1..5]
        .try_into()
        .map_err(|_| error::ConversionError::new())?;
    let brightness = u32::from_le_bytes(bytes_recieved);
    return Ok(brightness);
}

pub fn get_brightness_percent(
    studio_display_handle: &mut hidapi::HidDevice,
) -> Result<u8, error::DisplayError> {
    let value = (get_brightness(studio_display_handle)? - MIN_BRIGHTNESS) as f32;
    let value_percent = (value / BRIGHTNESS_RANGE as f32 * 100.0) as u8;
    return Ok(value_percent);
}

pub fn get_studio_display() -> Result<HidDevice, error::DisplayError> {
    let hapi = HidApi::new()?;

    let devices: Vec<&hidapi::DeviceInfo> = hapi
        .device_list()
        .filter(|x| {
            x.product_id() == SD_PRODUCT_ID
                && x.vendor_id() == SD_VENDOR_ID
                && x.interface_number() == SD_INTERFACE_NR
        })
        .collect();

    let Some(display) = devices.first() else {
        return Err(error::DisplayError::NoDeviceError(
            error::NoDeviceError::new("no first device".into()),
        ));
    };

    Ok(hapi.open_path(display.path())?)
}
