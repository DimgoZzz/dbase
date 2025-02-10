pub enum RenderBackend
{
	None,
	OpenGL,
	Vulkan,
}

impl Default for RenderBackend
{
	fn default() -> Self
	{
		#[cfg(all(feature = "backend_vulkan", feature = "backend_opengl"))]
		compile_error!("Error: Multiple render backends enabled");

		crate::common::cfg_if! {
			if #[cfg(feature = "backend_opengl")]
			{
				return Self::OpenGL;
			} else
			if #[cfg(feature = "backend_vulkan")]
			{
				return Self::Vulkan;
			}
			else{
				Self::None
			}
		}

	}
}
