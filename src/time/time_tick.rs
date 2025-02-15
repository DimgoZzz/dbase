use super::{DeltaTime,
            PerformanceCounter,
            TimeTick};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub struct TimeStamp
{
	pub tick : TimeTick,
}

impl TimeStamp
{
	pub fn new(tick : TimeTick) -> Self
	{
		Self { tick }
	}

	pub fn now() -> Self
	{
		Self::new(PerformanceCounter::count())
	}

	pub fn elapsed_since(self,
	                     since : TimeStamp)
	                     -> Self
	{
		self - since
	}
}

impl TimeStamp
{
	pub fn as_seconds(&self) -> DeltaTime
	{
		self.tick as DeltaTime / PerformanceCounter::frequency_per_sec() as DeltaTime
	}
}

impl Into<TimeTick> for TimeStamp
{
	fn into(self) -> TimeTick
	{
		self.tick
	}
}

impl From<TimeTick> for TimeStamp
{
	fn from(value : TimeTick) -> Self
	{
		Self::new(value)
	}
}

impl std::ops::Sub for TimeStamp
{
	type Output = Self;

	fn sub(self,
	       rhs : Self)
	       -> Self::Output
	{
		Self { tick : (self.tick - rhs.tick), }
	}
}

impl std::ops::Add for TimeStamp
{
	type Output = Self;

	fn add(self,
	       rhs : Self)
	       -> Self::Output
	{
		Self { tick : (self.tick + rhs.tick), }
	}
}

impl<'a> std::fmt::Display for TimeStamp
{
	fn fmt(&self,
	       f : &mut std::fmt::Formatter<'_>)
	       -> std::fmt::Result
	{
		// TODO: fix me
		let frequency = PerformanceCounter::frequency_per_sec();
		let mut ticks = self.tick;
		let ticks_in_min = frequency * 60;
		let ticks_in_ms = frequency / 1000;

		let m = ticks / ticks_in_min;
		ticks = ticks - m * ticks_in_min;

		let s = ticks / frequency;
		ticks = ticks - s * frequency;

		let ms = ticks / ticks_in_ms;
		ticks = ticks - ms * ticks_in_ms;
		write!(f, "{m:0>2}:{s:0>2}.{ms:0>3}|{ticks:0>6}")
	}
}
