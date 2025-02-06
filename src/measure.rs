use std::time::Instant;

#[derive(Debug)]
pub struct Measure {
    name: &'static str,
    start: Instant,
    duration: u64,
}