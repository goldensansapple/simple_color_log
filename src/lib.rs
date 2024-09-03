/*
Copyright 2024 Jesse Gomez

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

/// Log fatal then exit
#[macro_export]
macro_rules! log_fatal {
    ($msg:expr) => {
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
            .unwrap_or_default();

        eprintln!(
            "{} {}:{}:{}: {}",
            "\x1b[0;34m[\x1b[1;31mF\x1b[0;34m]\x1b[0m",
            file!().split("\\").last().unwrap_or(""),
            line!(),
            OffsetDateTime::now_utc()
                .format(&format)
                .unwrap_or_default(),
            $msg
        );

        panic!($msg);
    };
}

/// Log error
#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {
        use time::format_description;
        use time::OffsetDateTime;

        #[cfg(feature = "error")]
        {
            let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap_or_default();

            eprintln!(
                "{} {}:{}:{}: {}",
                "\x1b[0;34m[\x1b[0;31mE\x1b[0;34m]\x1b[0m",
                file!().split("\\").last().unwrap_or(""),
                line!(),
                OffsetDateTime::now_utc()
                    .format(&format)
                    .unwrap_or_default(),
                $msg
            );
        }
    };
}

/// Log warning
#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => {
        #[cfg(feature = "warn")]
        {
            let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap_or_default();

            eprintln!(
                "{} {}:{}:{}: {}",
                "\x1b[0;34m[\x1b[0;33mW\x1b[0;34m]\x1b[0m",
                file!().split("\\").last().unwrap_or(""),
                line!(),
                OffsetDateTime::now_utc()
                    .format(&format)
                    .unwrap_or_default(),
                $msg
            );
        }
    };
}

/// Log info
#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        #[cfg(feature = "info")]
        {
            let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap_or_default();

            println!(
                "{} {}:{}:{}: {}",
                "\x1b[0;34m[\x1b[0;32mI\x1b[0;34m]\x1b[0m",
                file!().split("\\").last().unwrap_or(""),
                line!(),
                OffsetDateTime::now_utc()
                    .format(&format)
                    .unwrap_or_default(),
                $msg
            );
        }
    };
}

/// Log debug
#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => {
        #[cfg(feature = "debug")]
        {
            let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap_or_default();

            println!(
                "{} {}:{}:{}: {}",
                "\x1b[0;34m[\x1b[0;36mD\x1b[0;34m]\x1b[0m",
                file!().split("\\").last().unwrap_or(""),
                line!(),
                OffsetDateTime::now_utc()
                    .format(&format)
                    .unwrap_or_default(),
                $msg
            );
        }
    };
}

/// Log trace
#[macro_export]
macro_rules! log_trace {
    ($msg:expr) => {
        #[cfg(feature = "trace")]
        {
            let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap_or_default();

            println!(
                "{} {}:{}:{}: {}",
                "\x1b[0;34m[\x1b[1;37mT\x1b[0;34m]\x1b[0m",
                file!().split("\\").last().unwrap_or(""),
                line!(),
                OffsetDateTime::now_utc()
                    .format(&format)
                    .unwrap_or_default(),
                $msg
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // log_fatal!("fatal");
        log_error!("error");
        log_warn!("warn");
        log_info!("info");
        log_debug!("debug");
        log_trace!("trace");
    }
}
