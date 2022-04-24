// Println implementations.
use core::fmt::Display;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut serial_port = ceros_serial::serial::Serial::new();
            let mut serial = ceros_serial::protocol::CEROSSerial::new(&mut serial_port);
            serial.write_data(ceros_serial::protocol::DataType::Print, format!("{}",format_args!($($arg)*)).as_bytes().to_vec());
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut serial_port = ceros_serial::serial::Serial::new();
            let mut serial = ceros_serial::protocol::CEROSSerial::new(&mut serial_port);
            serial.write_data(ceros_serial::protocol::DataType::Print, format!("{}\n",format_args!($($arg)*)).as_bytes().to_vec());
        }
    };
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut serial_port = ceros_serial::serial::Serial::new();
            let mut serial = ceros_serial::protocol::CEROSSerial::new(&mut serial_port);
            serial.write_data(ceros_serial::protocol::DataType::Error, format!("{}",format_args!($($arg)*)).as_bytes().to_vec());
        }
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut serial_port = ceros_serial::serial::Serial::new();
            let mut serial = ceros_serial::protocol::CEROSSerial::new(&mut serial_port);
            serial.write_data(ceros_serial::protocol::DataType::Error, format!("{}\n",format_args!($($arg)*)).as_bytes().to_vec());
        }
    };
}