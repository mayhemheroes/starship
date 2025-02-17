#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    starship::formatter::parse(data);
});
