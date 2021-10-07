use clap::Clap;
use std::ffi;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clap, Debug)]
struct Opts {
    #[clap()]
    host: String,
    #[clap()]
    tag: String,
    #[clap()]
    index: u16,
    #[clap()]
    value: Option<u8>,
    #[clap(long = "timeout", default_value = "5.0")]
    timeout: f64,
    #[clap(long = "path", default_value = "1,0")]
    path: String,
}

const SLEEP_STEP: Duration = Duration::from_millis(1);

fn main() {
    macro_rules! check_err {
        ($cond: expr, $result: expr) => {
            if $cond {
                panic!("libplctag error: {}", $result);
            }
        };
    }
    let opts = Opts::parse();
    let tag_path = format!(
        "protocol=ab_eip&gateway={}&path={}&cpu=LGX&name={}",
        opts.host, opts.path, opts.tag
    );
    let path = ffi::CString::new(tag_path.clone()).unwrap();
    let timeout = Duration::from_secs_f64(opts.timeout);
    let plc_timeout = timeout.as_millis() as i32;
    let tag_id = unsafe { plctag::plc_tag_create(path.as_ptr(), plc_timeout) };
    if tag_id < 0 {
        panic!("Unable to create tag request for {}: {}", tag_path, tag_id);
    }
    let start = Instant::now();
    loop {
        let rc = unsafe { plctag::plc_tag_status(tag_id) };
        match rc {
            plctag::PLCTAG_STATUS_PENDING => {
                if start.elapsed() > timeout {
                    panic!("Tag timeout");
                }
                thread::sleep(SLEEP_STEP);
            }
            plctag::PLCTAG_STATUS_OK => break,
            _ => {
                panic!("Unable to request tag {}: {}", tag_path, rc);
            }
        }
    }
    if let Some(v) = opts.value {
        let result = unsafe { plctag::plc_tag_set_bit(tag_id, opts.index as i32, v as i32) };
        check_err!(result != plctag::PLCTAG_STATUS_OK, result);
        println!("{}[{}] = {}", opts.tag, opts.index, v);
        let result = unsafe { plctag::plc_tag_write(tag_id, plc_timeout) };
        check_err!(result != plctag::PLCTAG_STATUS_OK, result);
    } else {
        let result = unsafe { plctag::plc_tag_get_bit(tag_id, opts.index as i32) };
        check_err!(result < 0, result);
        println!("{}", result);
    };
}
