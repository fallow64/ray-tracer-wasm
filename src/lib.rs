use crate::output::TargetOutput;

#[macro_use]
mod glue;
mod hittable;
mod math;
mod output;
mod scene;

#[unsafe(no_mangle)]
pub extern "C" fn create_renderer_target(width: u32, height: u32) -> &'static mut TargetOutput {
    let output = TargetOutput::empty(width, height);
    Box::leak(Box::new(output))
}

#[unsafe(no_mangle)]
pub extern "C" fn render(target: &mut TargetOutput) {
    target.render();
}
