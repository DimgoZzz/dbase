use crate::external::sdl2;
use crate::time::*;

pub struct PerformanceCounter {}

static mut START_TICK : TimeTick = 0;

impl PerformanceCounter
{
	#[inline]
	pub fn frequency_per_sec() -> TimeTick
	{
		unsafe { sdl2::sys::SDL_GetPerformanceFrequency() }
	}

	#[inline]
	pub fn count_absolute() -> TimeTick
	{
		unsafe { sdl2::sys::SDL_GetPerformanceCounter() }
	}

	#[inline]
	pub fn count() -> TimeTick
	{
		Self::count_absolute() - unsafe { START_TICK }
	}

	#[inline]
	pub fn reset()
	{
		unsafe {
			START_TICK = Self::count_absolute();
		};
	}

	#[inline]
	pub fn get_start_tick() -> TimeTick
	{
		unsafe { START_TICK }
	}
}
