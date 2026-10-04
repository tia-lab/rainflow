use crate::{MathError, MathResult, config::RAINFLOW_MAX_INPUT_LEN};
use std::collections::TryReserveError;



/// An excursion range, not a complete cycle's temporal duration.
/// Endpoint indices identify its extrema, not when the cycle became observable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RainflowCycle {
    pub range: f64,
    pub mean: f64,
    /// Exactly 0.5 for a half cycle or 1.0 for a full cycle.
    pub count: f64,
    pub start_index: usize,
    pub end_index: usize,
}

/// Count cycles in an ordered, finalized record without merging equal ranges.
///
/// Interior plateau extrema use their last index; the first/last samples are
/// endpoints. Residual excursions are half cycles. A constant record is empty.
/// Rejects lengths outside 2..=RAINFLOW_MAX_INPUT_LEN, non-finite inputs, and
/// unrepresentable ranges. O(n) time and space; borrows and never sorts input.
pub fn rainflow_cycles(values: &[f64]) -> MathResult<Vec<RainflowCycle>> {
    let n = values.len();
    if n < 2 {
        return Err(MathError::InsufficientDataAlgo {
            required: 2,
            actual: n,
        });
    }
    if n > RAINFLOW_MAX_INPUT_LEN {
        return Err(MathError::InvalidData(format!(
            "rainflow_cycles: input length exceeds {RAINFLOW_MAX_INPUT_LEN}"
        )));
    }
    let mut finite = true;
    let mut constant = true;
    for &value in values {
        finite &= value.is_finite();
        constant &= value == values[0];
    }
    if !finite {
        return Err(MathError::InvalidData(
            "rainflow_cycles: all inputs must be finite".into(),
        ));
    }
    if constant {
        return Ok(Vec::new());
    }

    let mut stack = Vec::new();
    let mut cycles = Vec::new();
    stack.try_reserve_exact(n).map_err(reservation_error)?;
    cycles.try_reserve_exact(n - 1).map_err(reservation_error)?;
    stack.push(0);
    let mut rising = None;
    for i in 1..n {
        if values[i] == values[i - 1] {
            continue;
        }
        let next = values[i] > values[i - 1];
        if rising.is_some_and(|previous| previous != next) {
            push_reversal(values, i - 1, &mut stack, &mut cycles)?;
        }
        rising = Some(next);
    }
    push_reversal(values, n - 1, &mut stack, &mut cycles)?;
    // Traverse residuals; repeated front-removal would make this quadratic.
    for pair in stack.windows(2) {
        let range = checked_range(values[pair[0]], values[pair[1]])?;
        cycles.push(make_cycle(values, pair[0], pair[1], 0.5, range)?);
    }
    Ok(cycles)
}

#[inline(always)]
fn push_reversal(
    values: &[f64],
    index: usize,
    stack: &mut Vec<usize>,
    cycles: &mut Vec<RainflowCycle>,
) -> MathResult<()> {
    stack.push(index);
    while stack.len() >= 3 {
        let n = stack.len();
        let (a, b, c) = (stack[n - 3], stack[n - 2], stack[n - 1]);
        let older = checked_range(values[a], values[b])?;
        let newer = checked_range(values[b], values[c])?;
        if newer < older {
            break;
        }
        let count = if n == 3 { 0.5 } else { 1.0 };
        cycles.push(make_cycle(values, a, b, count, older)?);
        if n == 3 {
            stack[0] = b;
            stack[1] = c;
            stack.truncate(2);
        } else {
            stack[n - 3] = c;
            stack.truncate(n - 2);
        }
    }
    Ok(())
}

fn checked_range(a: f64, b: f64) -> MathResult<f64> {
    let range = (b - a).abs();
    if !range.is_finite() {
        return Err(numerical_error("range is not representable"));
    }
    Ok(range)
}

// Internal visibility permits testing the defensive mean guard directly.
pub(super) fn make_cycle(
    values: &[f64],
    start_index: usize,
    end_index: usize,
    count: f64,
    range: f64,
) -> MathResult<RainflowCycle> {
    let (a, b) = (values[start_index], values[end_index]);
    let (lo, hi) = (a.min(b), a.max(b));
    let mean = if lo <= 0.0 && hi >= 0.0 {
        (lo + hi) * 0.5
    } else {
        lo + (hi - lo) * 0.5
    };
    if !mean.is_finite() {
        return Err(numerical_error("mean is not representable"));
    }
    Ok(RainflowCycle {
        range,
        mean: if mean == 0.0 { 0.0 } else { mean },
        count,
        start_index,
        end_index,
    })
}

fn numerical_error(reason: &str) -> MathError {
    MathError::NumericalError {
        reason: reason.into(),
        operation: Some("rainflow_cycles".into()),
    }
}

pub(super) fn reservation_error(_: TryReserveError) -> MathError {
    MathError::CalculationError("rainflow_cycles: buffer reservation failed".into())
}
