mod devices;

use headless_chrome::protocol::types::{JsInt, JsFloat};
use headless_chrome::protocol::Method;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};
use tide::log::info;
use tide::prelude::*;
use tide::Request;
use tide::{http::mime, Response};

use crate::devices::devices::get_device;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(health);
    app.at("/v1/screenshot").get(generate_screenshot);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn health(mut _req: Request<()>) -> tide::Result {
    let response = Response::builder(200).body(json!({"status": "OK"})).build();
    Ok(response.into())
}

#[derive(Deserialize)]
#[serde(default)]
struct ScreenshotOptions {
    url: String,
    device: String,
    quality: u8,
    delay: u8,
}

impl Default for ScreenshotOptions {
    fn default() -> Self {
        Self {
            delay: 0,
            device: "Galaxy S5".to_string(),
            url: "https://google.com".to_string(),
            quality: 80,
        }
    }
}

// https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDeviceMetricsOverride
// https://github.com/puppeteer/puppeteer/blob/7001322cd1cf9f77ee2c370d50a6707e7aaad72d/src/common/EmulationManager.ts#L44
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
struct EmulationDeviceMetricsOverride {
    width: JsInt,
    height: JsInt,
    device_scale_factor: JsFloat,
    mobile: bool,
}

impl Default for EmulationDeviceMetricsOverride {
    fn default() -> Self {
        EmulationDeviceMetricsOverride {
            width: 360,
            height: 640,
            device_scale_factor: 3.0,
            mobile: true,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct EmulationDeviceMetricsOverrideReturnObject {}

impl Method for EmulationDeviceMetricsOverride {
    const NAME: &'static str = "Emulation.setDeviceMetricsOverride";
    type ReturnObject = EmulationDeviceMetricsOverrideReturnObject;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
struct EmulationUserAgent<'a> {
    user_agent: &'a str,
}

impl<'a> Default for EmulationUserAgent<'a> {
    fn default() -> Self {
        EmulationUserAgent {
            user_agent: "Mozilla/5.0 (Linux; Android 5.0; SM-G900P Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3765.0 Mobile Safari/537.36"
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct EmulationUserAgentReturnObject {}

impl<'a> Method for EmulationUserAgent<'a> {
    const NAME: &'static str = "Emulation.setUserAgentOverride";
    type ReturnObject = EmulationUserAgentReturnObject;
}

async fn generate_screenshot(req: Request<()>) -> tide::Result {
    let opt: ScreenshotOptions = req.query()?;
    let browser = Browser::new(LaunchOptionsBuilder::default().build().unwrap()).unwrap();
    let device = get_device(&opt.device).unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();
    info!("{}", &opt.url);
    tab.call_method(EmulationDeviceMetricsOverride {
        device_scale_factor: device.device_scale_factor,
        mobile: device.is_mobile,
        width: device.width,
        height: device.height,
    })
    .unwrap();
    tab.call_method(EmulationUserAgent {
        user_agent: device.user_agent
    }).unwrap();
    tab.navigate_to(&opt.url).unwrap();

    let _jpeg_data = tab
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)
        .unwrap();

    let r = Response::builder(200)
        .content_type(mime::JPEG)
        .body(_jpeg_data)
        .build();
    Ok(r.into())
}
