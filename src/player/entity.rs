use Rect;
use std::any::Any;
use Window;
use Canvas;

pub trait Entity {
    // Reference: https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
    fn as_any(&self) -> &dyn Any;
    fn id(&self) -> u32;
    fn collider(&self) -> Rect;
    fn render(&self, canvas: &mut Canvas<Window>) -> bool;
}

