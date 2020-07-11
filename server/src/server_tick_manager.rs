use std::time::Duration;

use naia_shared::HostTickManager;

/// Manages the current tick for the host
#[derive(Debug)]
pub struct ServerTickManager {
    tick_interval: Duration,
    current_tick: u16,
}

impl ServerTickManager {
    /// Create a new HostTickManager with a given tick interval duration
    pub fn new(tick_interval: Duration) -> Self {
        ServerTickManager {
            tick_interval,
            current_tick: 0,
        }
    }

    /// Increments the current tick
    pub fn increment_tick(&mut self) {
        self.current_tick = self.current_tick.wrapping_add(1);
    }
}

impl HostTickManager for ServerTickManager {
    fn get_tick(&self) -> u16 {
        self.current_tick
    }

    fn process_incoming(&mut self, tick_latency: i8) {
        // Should not ever arrive here
        // unimplemented!()
    }
}