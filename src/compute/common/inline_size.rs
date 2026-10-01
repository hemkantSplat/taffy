//! A box's inline size is final before its contents are laid out
use crate::geometry::Size;
use crate::tree::{LayoutInput, LayoutOutput, RequestedAxis, RunMode};

/// Lays a container out at its final inline size (CSS 2 10.3 before 10.6): an open width is sized alone first, with
/// cyclic percentages inside (css-sizing-3 5.2.1), then the contents lay out against it as a definite size.
pub(crate) fn compute_at_final_inline_size(
    inputs: LayoutInput,
    mut compute: impl FnMut(LayoutInput) -> LayoutOutput,
) -> LayoutOutput {
    let width_only = inputs.run_mode == RunMode::ComputeSize && inputs.axis == RequestedAxis::Horizontal;
    if width_only || inputs.known_dimensions.width.is_some() || inputs.run_mode == RunMode::PerformHiddenLayout {
        return compute(inputs);
    }
    let width =
        compute(LayoutInput { run_mode: RunMode::ComputeSize, axis: RequestedAxis::Horizontal, ..inputs }).size.width;
    compute(LayoutInput {
        known_dimensions: Size { width: Some(width), ..inputs.known_dimensions },
        known_dimensions_are_definite: Size { width: true, ..inputs.known_dimensions_are_definite },
        ..inputs
    })
}
