#![feature(prelude_import)]
#![no_std]
/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
//!  Sensor app that reads sensor data from a temperature sensor and sends the sensor data to a CoAP server over NB-IoT.
//!  Note that we are using a patched version of apps/my_sensor_app/src/vsscanf.c that
//!  fixes AT response parsing bugs.  The patched file must be present in that location.
//!  This is the Rust version of `https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/rust-nbiot/apps/my_sensor_app/OLDsrc/main.c`

#![no_std]
//  Don't link with standard Rust library, which is not compatible with embedded systems
#![feature(trace_macros)]
//  Allow macro tracing: `trace_macros!(true)`
#![feature(concat_idents)]
//  Allow `concat_idents!()` macro used in `coap!()` macro
#![feature(const_transmute)]
//  Allow `transmute` for initialising Mynewt structs
#![feature(proc_macro_hygiene)]
//  Allow Procedural Macros like `run!()`
#![feature(custom_attribute)]
#[prelude_import]
use ::core::prelude::v1::*;
#[macro_use]
extern crate core as core;
#[macro_use]
extern crate compiler_builtins as compiler_builtins;
//  Allow Custom Attributes like `#[safe_wrap]`

extern crate cortex_m;
//  Declare the external library `cortex_m`
extern crate mynewt;
//  Declare the Mynewt library
extern crate macros as mynewt_macros;
//  Declare the Mynewt Procedural Macros library

mod mynewt_hal {
    //  Declare `mynewt_hal.rs` as Rust module `mynewt_hal` for Mynewt Rust Embedded HAL functions
    //  Declare `app_network.rs` as Rust module `app_network` for Application Network functions
    //  Declare `app_sensor.rs` as Rust module `app_sensor` for Application Sensor functions
    //  Declare `touch_sensor.rs` as Rust module `touch_sensor` for Touch Sensor functions
    //  Declare `display.rs` as Rust module `display` for Display functions

    //  If floating-point is enabled...
    //  Declare `gps_sensor.rs` as Rust module `gps_sensor` for GPS Sensor functions

    //  Import `PanicInfo` type which is used by `panic()` below
    //  Import cortex_m assembly function to inject breakpoint
    //  Import Mynewt OS API
    //  Import Mynewt Console API
    //libs::sensor_network,   //  Import Mynewt Sensor Network Library


    //  Don't mangle the name "main"
    //  Declare extern "C" because it will be called by Mynewt
    //  Initialise the Mynewt packages and internal temperature sensor driver. Any startup
    //  functions defined in pkg.yml of our custom drivers and libraries will be called by 
    //  sysinit().  Here are the startup functions consolidated by Mynewt:
    //  bin/targets/nrf52_my_sensor/generated/src/nrf52_my_sensor-sysinit-app.c


    //  Start the Server Transport for sending sensor data to CoAP Server over NB-IoT.
    //sensor_network::start_server_transport()
    //.expect("NET fail");

    //  Start polling the temperature sensor every 10 seconds in the background.

    //  Start polling the GPS.
    //gps_sensor::start_gps_listener()
    //.expect("GPS fail");

    //  Start Bluetooth Beacon.  TODO: Create a safe wrapper for starting Bluetooth LE.

    //  Show the display.

    //  Test the touch sensor.

    //  Main event loop
    //  Loop forever...
    //  Processing events...
    //  From default event queue.
    //  Never comes here

    //  Display the filename and line number to the Semihosting Console.
    //  Pause in the debugger.
    //  Loop forever so that device won't restart.
    use embedded_hal;
    use mynewt::{hw::hal, kernel::os};
    /// Rust Embedded HAL interface for Mynewt SPI
    impl MynewtSPI {
        /// Create a new SPI port
        pub fn new(spi_num: i32, cs_pin: i32,
                   spi_settings: *mut hal::hal_spi_settings) -> Self {
            let rc = unsafe { hal::hal_spi_config(spi_num, spi_settings) };
            {
                match (&(rc), &(0)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                              "`,\n right: `",
                                                                                              "`: "],
                                                                                            &match (&&*left_val,
                                                                                                    &&*right_val,
                                                                                                    &::core::fmt::Arguments::new_v1(&["spi config fail"],
                                                                                                                                    &match ()
                                                                                                                                         {
                                                                                                                                         ()
                                                                                                                                         =>
                                                                                                                                         [],
                                                                                                                                     }))
                                                                                                 {
                                                                                                 (arg0,
                                                                                                  arg1,
                                                                                                  arg2)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg2,
                                                                                                                               ::core::fmt::Display::fmt)],
                                                                                             }),
                                                             &("rust/app/src/mynewt_hal.rs",
                                                               12u32, 9u32))
                            }
                        }
                    }
                }
            };
            let rc = unsafe { hal::hal_spi_enable(spi_num) };
            {
                match (&(rc), &(0)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                              "`,\n right: `",
                                                                                              "`: "],
                                                                                            &match (&&*left_val,
                                                                                                    &&*right_val,
                                                                                                    &::core::fmt::Arguments::new_v1(&["spi enable fail"],
                                                                                                                                    &match ()
                                                                                                                                         {
                                                                                                                                         ()
                                                                                                                                         =>
                                                                                                                                         [],
                                                                                                                                     }))
                                                                                                 {
                                                                                                 (arg0,
                                                                                                  arg1,
                                                                                                  arg2)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg2,
                                                                                                                               ::core::fmt::Display::fmt)],
                                                                                             }),
                                                             &("rust/app/src/mynewt_hal.rs",
                                                               15u32, 9u32))
                            }
                        }
                    }
                }
            };
            let rc = unsafe { hal::hal_gpio_init_out(cs_pin, 1) };
            {
                match (&(rc), &(0)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                              "`,\n right: `",
                                                                                              "`: "],
                                                                                            &match (&&*left_val,
                                                                                                    &&*right_val,
                                                                                                    &::core::fmt::Arguments::new_v1(&["spi init fail"],
                                                                                                                                    &match ()
                                                                                                                                         {
                                                                                                                                         ()
                                                                                                                                         =>
                                                                                                                                         [],
                                                                                                                                     }))
                                                                                                 {
                                                                                                 (arg0,
                                                                                                  arg1,
                                                                                                  arg2)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg2,
                                                                                                                               ::core::fmt::Display::fmt)],
                                                                                             }),
                                                             &("rust/app/src/mynewt_hal.rs",
                                                               18u32, 9u32))
                            }
                        }
                    }
                }
            };
            MynewtSPI{spi_num, cs_pin,}
        }
    }
    /// Rust Embedded HAL interface for Mynewt SPI
    impl embedded_hal::blocking::spi::Write<u8> for MynewtSPI {
        /// Write to the SPI port
        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            unsafe { hal::hal_gpio_write(self.cs_pin, 0) };
            unsafe {
                hal::hal_spi_txrx(self.spi_num,
                                  core::mem::transmute(words.as_ptr()),
                                  core::ptr::null_mut(), words.len() as i32)
            };
            unsafe { hal::hal_gpio_write(self.cs_pin, 1) };
            Ok(())
        }
        /// Reuse Mynewt error codes
        type
        Error
        =
        mynewt::result::MynewtError;
    }
    /// Rust Embedded HAL interface for Mynewt GPIO
    impl MynewtGPIO {
        /// Create a new output GPIO pin
        pub fn new(pin: i32) -> Self {
            let rc = unsafe { hal::hal_gpio_init_out(pin, 0) };
            {
                match (&(rc), &(0)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                              "`,\n right: `",
                                                                                              "`: "],
                                                                                            &match (&&*left_val,
                                                                                                    &&*right_val,
                                                                                                    &::core::fmt::Arguments::new_v1(&["gpio fail"],
                                                                                                                                    &match ()
                                                                                                                                         {
                                                                                                                                         ()
                                                                                                                                         =>
                                                                                                                                         [],
                                                                                                                                     }))
                                                                                                 {
                                                                                                 (arg0,
                                                                                                  arg1,
                                                                                                  arg2)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg2,
                                                                                                                               ::core::fmt::Display::fmt)],
                                                                                             }),
                                                             &("rust/app/src/mynewt_hal.rs",
                                                               53u32, 9u32))
                            }
                        }
                    }
                }
            };
            MynewtGPIO{pin,}
        }
    }
    /// Rust Embedded HAL interface for Mynewt GPIO
    impl embedded_hal::digital::v2::OutputPin for MynewtGPIO {
        /// Set the GPIO pin to low
        fn set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { hal::hal_gpio_write(self.pin, 0) };
            Ok(())
        }
        /// Set the GPIO pin to high
        fn set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { hal::hal_gpio_write(self.pin, 1) };
            Ok(())
        }
        /// Reuse Mynewt error codes
        type
        Error
        =
        mynewt::result::MynewtError;
    }
    /// Rust Embedded HAL interface for Mynewt Delay
    impl embedded_hal::blocking::delay::DelayMs<u8> for MynewtDelay {
        /// Sleep for the specified number of milliseconds
        fn delay_ms(&mut self, ms: u8) {
            const OS_TICKS_PER_SEC: u32 = 1000;
            let delay_ticks = (ms as u32) * OS_TICKS_PER_SEC / 1000;
            unsafe { os::os_time_delay(delay_ticks) };
        }
    }
    /// Rust Embedded HAL interface for Mynewt SPI
    pub struct MynewtSPI {
        /// Mynewt SPI port number
        spi_num: i32,
        /// Mynewt GPIO pin number for Chip Select
        cs_pin: i32,
    }
    /// Rust Embedded HAL interface for Mynewt GPIO
    pub struct MynewtGPIO {
        /// Mynewt GPIO pin number
        pin: i32,
    }
    /// Rust Embedded HAL interface for Mynewt Delay
    pub struct MynewtDelay {
    }
}
mod app_network {
    //!  Transmit sensor data to a CoAP server like thethings.io.  The CoAP payload will be encoded as JSON.
    //!  The sensor data will be transmitted over NB-IoT.
    //!  Note that we are using a patched version of apps/my_sensor_app/src/vsscanf.c that
    //!  fixes response parsing bugs.  The patched file must be present in that location.
    //!  This is the Rust version of `https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/rust-nbiot/apps/my_sensor_app/OLDsrc/network.c`
    use mynewt::{result::*, hw::sensor::{SensorValue, SensorValueType},
                 sys::console, encoding::coap_context::*,
                 libs::{sensor_network}, coap, d, Strn};
    use mynewt_macros::strn;
    #[cfg(not(feature = "use_float"))]
    pub fn aggregate_sensor_data(sensor_value: &SensorValue)
     -> MynewtResult<()> {
        send_sensor_data(sensor_value)
    }
    /// Compose a CoAP JSON message with the Sensor Key (field name), Value and Geolocation (optional) in `val`
    /// and send to the CoAP server.  The message will be enqueued for transmission by the CoAP / OIC 
    /// Background Task so this function will return without waiting for the message to be transmitted.
    /// Return `Ok()` if successful, `SYS_EAGAIN` if network is not ready yet.
    /// For the CoAP server hosted at thethings.io, the CoAP payload shall be encoded in JSON like this:
    /// ```json
    /// {"values":[
    ///   {"key":"t",      "value":1715, "geo": { "lat": ..., "long": ... }},
    ///   {"key":"device", "value":"0102030405060708090a0b0c0d0e0f10"}
    /// ]}
    /// ```
    fn send_sensor_data(val: &SensorValue) -> MynewtResult<()> {
        console::print("Rust send_sensor_data: ");
        if let SensorValueType::Uint(i) = val.value {
            console::print_strn(val.key);
            console::print("=");
            console::printint(i as i32);
        }
        console::print("\n");
        console::flush();
        let device_id = sensor_network::get_device_id()?;
        let rc = sensor_network::init_server_post(&Strn::new(b"\0"))?;
        if !rc { return Err(MynewtError::SYS_EAGAIN); }
        let _payload =
            {
                "begin json root";
                {
                    "begin json coap_root";
                    unsafe {
                        mynewt::libs::sensor_network::prepare_post(mynewt::encoding::APPLICATION_JSON)?;
                    }
                    unsafe {
                        mynewt::libs::sensor_coap::json_rep_start_root_object();
                    }
                    {
                        {
                            "begin json coap_array , object : COAP_CONTEXT , key : values";
                            {
                                "<< jarri , o: COAP_CONTEXT, k: values";
                                let key_with_null: &str = "values\u{0}";
                                unsafe {
                                    mynewt::libs::mynewt_rust::json_helper_set_array(COAP_CONTEXT.to_void_ptr(),
                                                                                     COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()));
                                };
                            };
                            {
                                " >>  >> val >> , \"device\" : & device_id ,";
                                "--------------------";
                                {
                                    "begin json coap_item_int_val , c : COAP_CONTEXT , val : val";
                                    let geo = val.geo;
                                    if let SensorValueType::Uint(val) =
                                           val.value {
                                        {
                                            "begin json coap_item_int , key : val.key , value : val";
                                            {
                                                "begin json coap_item , array : COAP_CONTEXT";
                                                {
                                                    "<< jitmi c: COAP_CONTEXT";
                                                    let key_with_null: &str =
                                                        "COAP_CONTEXT\u{0}";
                                                    unsafe {
                                                        mynewt::libs::mynewt_rust::json_helper_object_array_start_item(COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                                    };
                                                };
                                                {
                                                    {
                                                        "-- jtxte o: COAP_CONTEXT, k: \"key\", v: val.key";
                                                        let key_with_opt_null:
                                                                &[u8] =
                                                            "key".to_bytes_optional_nul();
                                                        let value_with_opt_null:
                                                                &[u8] =
                                                            val.key.to_bytes_optional_nul();
                                                        unsafe {
                                                            mynewt::libs::mynewt_rust::json_helper_set_text_string(COAP_CONTEXT.to_void_ptr(),
                                                                                                                   COAP_CONTEXT.key_to_cstr(key_with_opt_null),
                                                                                                                   COAP_CONTEXT.value_to_cstr(value_with_opt_null))
                                                        };
                                                    };
                                                    {
                                                        "-- jinte o: COAP_CONTEXT, k: \"value\", v: val";
                                                        let key_with_opt_null:
                                                                &[u8] =
                                                            "value".to_bytes_optional_nul();
                                                        let value =
                                                            val as u64;
                                                        unsafe {
                                                            mynewt::libs::mynewt_rust::json_helper_set_int(COAP_CONTEXT.to_void_ptr(),
                                                                                                           COAP_CONTEXT.key_to_cstr(key_with_opt_null),
                                                                                                           value)
                                                        };
                                                    };
                                                    unsafe {
                                                        COAP_CONTEXT.json_set_geolocation(&Strn::new(b"geo\x00"),
                                                                                          &Strn::new(b"lat\x00"),
                                                                                          &Strn::new(b"long\x00"),
                                                                                          geo)
                                                    };
                                                };
                                                {
                                                    ">>";
                                                    let key_with_null: &str =
                                                        "COAP_CONTEXT\u{0}";
                                                    unsafe {
                                                        mynewt::libs::mynewt_rust::json_helper_object_array_end_item(COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                                    };
                                                };
                                                "end json coap_item";
                                            };
                                            "end json coap_item_int";
                                        };
                                    } else {
                                        unsafe {
                                            COAP_CONTEXT.fail(CoapError::VALUE_NOT_UINT)
                                        };
                                    }
                                    "end json coap_item_int_val";
                                };
                                "--------------------";
                                " >>  >> \"device\" >> : & device_id ,";
                                "add1 key : \"device\" value : $crate::parse!(@ json &device_id) to object :\nCOAP_CONTEXT";
                                {
                                    "begin json coap_item_str , parent : COAP_CONTEXT , key : \"device\" , val :\n$crate::parse!(@ json &device_id)";
                                    {
                                        "begin json coap_item , array : COAP_CONTEXT";
                                        {
                                            "<< jitmi c: COAP_CONTEXT";
                                            let key_with_null: &str =
                                                "COAP_CONTEXT\u{0}";
                                            unsafe {
                                                mynewt::libs::mynewt_rust::json_helper_object_array_start_item(COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                            };
                                        };
                                        {
                                            {
                                                "-- jtxti o: COAP_CONTEXT, k: key, v: \"device\"";
                                                let key_strn: &Strn =
                                                    &Strn::new(b"key\x00");
                                                let value_strn: &Strn =
                                                    &Strn::new(b"device\x00");
                                                unsafe {
                                                    COAP_CONTEXT.json_set_text_string(key_strn,
                                                                                      value_strn)
                                                };
                                            };
                                            {
                                                "-- jtxti o: COAP_CONTEXT, k: value, v: $crate::parse!(@ json &device_id)";
                                                let key_strn: &Strn =
                                                    &Strn::new(b"value\x00");
                                                let value_strn: &Strn =
                                                    &device_id;
                                                unsafe {
                                                    COAP_CONTEXT.json_set_text_string(key_strn,
                                                                                      value_strn)
                                                };
                                            };
                                        };
                                        {
                                            ">>";
                                            let key_with_null: &str =
                                                "COAP_CONTEXT\u{0}";
                                            unsafe {
                                                mynewt::libs::mynewt_rust::json_helper_object_array_end_item(COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                            };
                                        };
                                        "end json coap_item";
                                    };
                                    "end json coap_item_str";
                                };
                                "--------------------";
                            };
                            {
                                ">>";
                                let key_with_null: &str = "values\u{0}";
                                unsafe {
                                    mynewt::libs::mynewt_rust::json_helper_close_array(COAP_CONTEXT.to_void_ptr(),
                                                                                       COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                };
                            };
                            "end json coap_array";
                        };
                    };
                    unsafe {
                        mynewt::libs::sensor_coap::json_rep_end_root_object();
                    }
                    "end json coap_root";
                };
                "end json root";
                ()
            };
        sensor_network::do_server_post()?;
        console::print("NET view your sensor at \nhttps://blue-pill-geolocate.appspot.com?device=");
        console::print_strn(&device_id);
        console::print("\n");
        Ok(())
    }
}
mod app_sensor {
    //!  Poll the temperature sensor every 10 seconds. Transmit the sensor data to the CoAP server after polling.
    //!  This is the Rust version of https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/nrf52/apps/my_sensor_app/OLDsrc/sensor.c
    use mynewt::{result::*, hw::sensor_mgr, hw::sensor::{self, sensor_type_t},
                 sys::console, Strn};
    use mynewt_macros::{init_strn};
    use crate::app_network;
    ///  Sensor to be polled: `temp_stub_0` is the stub temperature sensor that simulates a temperature sensor
    static SENSOR_DEVICE: Strn =
        Strn{rep: mynewt::StrnRep::ByteStr(b"temp_stub_0\x00"),};
    ///  Poll sensor every 10,000 milliseconds (10 seconds)  
    const SENSOR_POLL_TIME: u32 = (10 * 1000);
    ///  Use key (field name) `t` to transmit raw temperature to CoAP Server
    const TEMP_SENSOR_KEY: Strn =
        Strn{rep: mynewt::StrnRep::ByteStr(b"t\x00"),};
    ///  Type of sensor: Raw temperature sensor (integer sensor values 0 to 4095)
    const TEMP_SENSOR_TYPE: sensor_type_t =
        sensor::SENSOR_TYPE_AMBIENT_TEMPERATURE_RAW;
    ///  Ask Mynewt to poll or read the temperature sensor and call `aggregate_sensor_data()`
    ///  Return `Ok()` if successful, else return `Err()` with `MynewtError` error code inside.
    pub fn start_sensor_listener() -> MynewtResult<()> {
        console::print("Rust TMP poll\n");
        let sensor =
            sensor_mgr::find_bydevname(&SENSOR_DEVICE).next().expect("no TMP");
        sensor::set_poll_rate_ms(&SENSOR_DEVICE, SENSOR_POLL_TIME)?;
        let listener =
            sensor::new_sensor_listener(&TEMP_SENSOR_KEY, TEMP_SENSOR_TYPE,
                                        app_network::aggregate_sensor_data)?;
        sensor::register_listener(sensor, listener)?;
        Ok(())
    }
}
mod touch_sensor {
    use embedded_hal::{self, blocking::delay::DelayMs,
                       digital::v2::OutputPin};
    use mynewt::{result::*, hw::hal, kernel::os, sys::console};
    use crate::mynewt_hal::{MynewtDelay, MynewtGPIO};
    /// Probe the I2C bus
    pub fn probe() -> MynewtResult<()> {
        for addr in 0..128 {
            let rc = unsafe { hal::hal_i2c_master_probe(1, addr, 1000) };
            if rc != hal::HAL_I2C_ERR_ADDR_NACK as i32 {
                console::print("0x");
                console::printhex(addr);
                console::print(": ");
                console::printhex(rc as u8);
                console::print("\n");
                console::flush();
            }
        }
        console::print("Done\n");
        console::flush();
        Ok(())
    }
    pub fn test() -> MynewtResult<()> {
        let mut delay = MynewtDelay{};
        let mut reset = MynewtGPIO::new(10);
        reset.set_low()?;
        delay.delay_ms(20);
        reset.set_high()?;
        delay.delay_ms(200);
        delay.delay_ms(200);
        for register in &[0x00, 0xA3, 0x9F, 0x8F, 0xA6, 0xA8] {
            for addr in 0..128 { read_register(addr, *register)?; }
        }
        console::print("Done\n");
        console::flush();
        Ok(())
    }
    fn read_register(addr: u8, register: u8) -> MynewtResult<()> {
        unsafe {
            I2C_BUFFER[0] = register;
            I2C_DATA.address = addr;
            I2C_DATA.len = I2C_BUFFER.len() as u16;
            I2C_DATA.buffer = I2C_BUFFER.as_mut_ptr();
        };
        let rc1 =
            unsafe { hal::hal_i2c_master_write(1, &mut I2C_DATA, 1000, 0) };
        if rc1 == hal::HAL_I2C_ERR_ADDR_NACK as i32 { return Ok(()); }
        unsafe {
            I2C_BUFFER[0] = 0x00;
            I2C_DATA.address = addr;
            I2C_DATA.len = I2C_BUFFER.len() as u16;
            I2C_DATA.buffer = I2C_BUFFER.as_mut_ptr();
        };
        let rc2 =
            unsafe { hal::hal_i2c_master_read(1, &mut I2C_DATA, 1000, 1) };
        if rc2 == hal::HAL_I2C_ERR_ADDR_NACK as i32 { return Ok(()); }
        console::print("addr: ");
        console::printhex(addr);
        console::print(", reg: ");
        console::printhex(register);
        console::print(" = ");
        console::printhex(unsafe { I2C_BUFFER[0] });
        console::print("\n");
        console::flush();
        Ok(())
    }
    static mut I2C_BUFFER: [u8; 1] = [0];
    static mut I2C_DATA: hal::hal_i2c_master_data =
        hal::hal_i2c_master_data{address: 0,
                                 len: 0,
                                 buffer: core::ptr::null_mut(),};
}
mod display {
    use embedded_graphics::{prelude::*, fonts, pixelcolor::Rgb565,
                            primitives::Circle};
    use embedded_hal::{self, digital::v2::OutputPin};
    use st7735_lcd::{self, Orientation};
    use mynewt::{result::*, hw::hal};
    use crate::mynewt_hal::{MynewtDelay, MynewtGPIO, MynewtSPI};
    /// SPI settings for ST7789 display controller
    static mut SPI_SETTINGS: hal::hal_spi_settings =
        hal::hal_spi_settings{data_order: hal::HAL_SPI_MSB_FIRST as u8,
                              data_mode: hal::HAL_SPI_MODE3 as u8,
                              baudrate: 8000,
                              word_size: hal::HAL_SPI_WORD_SIZE_8BIT as u8,};
    /// Render the ST7789 display connected to SPI port 0
    pub fn show() -> MynewtResult<()> {
        let spi = MynewtSPI::new(0, 25, unsafe { &mut SPI_SETTINGS });
        let dc = MynewtGPIO::new(18);
        let rst = MynewtGPIO::new(26);
        let mut display = st7735_lcd::ST7735::new(spi, dc, rst, false, true);
        let mut backlight = MynewtGPIO::new(23);
        backlight.set_low()?;
        let c =
            Circle::<Rgb565>::new(Coord::new(40, 40),
                                  40).fill(Some(Rgb565::from(1u8)));
        let t =
            fonts::Font12x16::<Rgb565>::render_str("I AM RUSTY BEACON").fill(Some(Rgb565::from(20u8))).translate(Coord::new(20,
                                                                                                                            16));
        let mut delay = MynewtDelay{};
        display.init(&mut delay)?;
        display.set_orientation(&Orientation::Landscape)?;
        display.set_offset(1, 25);
        display.draw(c);
        display.draw(t);
        Ok(())
    }
}
use core::panic::PanicInfo;
use cortex_m::asm::bkpt;
use mynewt::{kernel::os, sys::console};
///  Main program that initialises the sensor, network driver and starts reading and sending sensor data in the background.
///  main() will be called at Mynewt startup. It replaces the C version of the main() function.
#[no_mangle]
extern "C" fn main() -> ! {
    mynewt::sysinit();
    app_sensor::start_sensor_listener().expect("TMP fail");
    extern "C" {
        fn start_ble() -> i32;
    }
    let rc = unsafe { start_ble() };
    if !(rc == 0) {
        {
            ::core::panicking::panic(&("BLE fail", "rust/app/src/lib.rs",
                                       77u32, 5u32))
        }
    };
    display::show().expect("DSP fail");
    touch_sensor::test().expect("TCH fail");
    loop  {
        os::eventq_run(os::eventq_dflt_get().expect("GET fail")).expect("RUN fail");
    }
}
///  This function is called on panic, like an assertion failure. We display the filename and line number and pause in the debugger. From https://os.phil-opp.com/freestanding-rust-binary/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    console::print("panic ");
    if let Some(location) = info.location() {
        let file = location.file();
        let line = location.line();
        console::print("at ");
        console::buffer(&file);
        console::print(" line ");
        console::printint(line as i32);
        console::print("\n");
        console::flush();
    } else { console::print("no loc\n"); console::flush(); }
    bkpt();
    loop  { }
}
