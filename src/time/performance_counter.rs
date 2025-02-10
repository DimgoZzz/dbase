use crate::external::sdl2;
use crate::time::*;

pub struct PerformanceCounter {}


impl PerformanceCounter
{
	#[inline]
	pub fn frequency_per_sec() -> TimeTick
	{
		unsafe { sdl2::sys::SDL_GetPerformanceFrequency() }
	}

	#[inline]
	pub fn count() -> TimeTick
	{
		unsafe { sdl2::sys::SDL_GetPerformanceCounter() }
	}

}
