use crate::time::*;

pub struct DeltaClock
{
	last_stamp : TimeStamp,
	delta :      TimeStamp,
}

impl DeltaClock
{
	pub fn new() -> Self
	{
		Self { last_stamp : TimeStamp::from_now(),
		       delta :      TimeStamp::new(1), }
	}

	pub fn tick(&mut self)
	{
		let new_stamp = TimeStamp::from_now();
		self.delta = new_stamp - self.last_stamp;
		self.last_stamp = new_stamp;
	}

	pub fn get_delta(&self) -> DeltaTime
	{
		self.delta.as_seconds()
	}

	pub fn get_delta_tick(&self) -> TimeStamp
	{
		self.delta
	}
}
