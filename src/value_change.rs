use bevy::prelude::*;

/// An event that indicates a change in value of a property. This is used by sliders, spinners
/// and other widgets that edit a value. The value is not clamped, so the receiver of the event
/// is responsible for clamping the value to the appropriate range. This is done to allow
/// the receiver to quantize the value or otherwise modify it before clamping.
#[derive(Clone, Debug)]
pub struct ValueChange<T>(pub T);

impl<T: Send + Sync + 'static> Event for ValueChange<T> {
    type Traversal = &'static ChildOf;

    const AUTO_PROPAGATE: bool = true;
}
