pub mod devices {
    use headless_chrome::protocol::types::{JsInt, JsFloat};
    use phf::phf_map;

    #[derive(Clone)]
    pub struct Device<'a> {
        pub name: &'a str,
        pub is_mobile: bool,
        pub user_agent: &'a str,
        pub has_touch: bool,
        pub width: i32,
        pub height: i32,
        pub device_scale_factor: JsFloat,
    }

    pub static DEVICES: phf::Map<&'static str, Device> = phf_map! {
        "Galaxy S5" => Device {
            name: "Galaxy S5",
            is_mobile: true,
            user_agent: "Mozilla/5.0 (Linux; Android 5.0; SM-G900P Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3765.0 Mobile Safari/537.36",
            has_touch: true,
            width: 360,
            height: 640,
            device_scale_factor: 3.0,
        },
        "Galaxy Tab S4" => Device {
            name: "Galaxy Tab S4",
            is_mobile: true,
            user_agent: "Mozilla/5.0 (Linux; Android 8.1.0; SM-T837A) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.80 Safari/537.36",
            has_touch: true,
            width: 712,
            height: 1138,
            device_scale_factor: 2.25,
        },
        "iPad" => Device {
            name: "iPad",
            is_mobile: true,
            user_agent: "Mozilla/5.0 (iPad; CPU OS 11_0 like Mac OS X) AppleWebKit/604.1.34 (KHTML, like Gecko) Version/11.0 Mobile/15A5341f Safari/604.1",
            has_touch: true,
            width: 768,
            height: 1024,
            device_scale_factor: 2.0,
        },
        "iPhone 5" => Device {
            name: "iPhone 5",
            is_mobile: true,
            user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3_1 like Mac OS X) AppleWebKit/603.1.30 (KHTML, like Gecko) Version/10.0 Mobile/14E304 Safari/602.1",
            has_touch: true,
            width: 320,
            height: 568,
            device_scale_factor: 2.0,
        }
    };
    pub fn get_device(name: &str) -> Option<Device> {
        DEVICES.get(name).or(DEVICES.get("Galaxy S5")).cloned()
    }
}
