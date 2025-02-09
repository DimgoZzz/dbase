pub type TimeTick = u64;

pub type DeltaTime = f64;

mod time_tick;
pub use time_tick::*;

mod performance_counter;
pub use performance_counter::*;

mod delta_clock;
pub use delta_clock::*;
