use crate::external::sdl3;
use crate::time::*;

pub struct PerformanceCounter {}

impl PerformanceCounter
{
	#[inline]
	pub fn frequency_per_sec() -> TimeTick
	{
		sdl3::timer::performance_frequency()
	}

	#[inline]
	pub fn count() -> TimeTick
	{
		sdl3::timer::performance_counter()
	}
}
