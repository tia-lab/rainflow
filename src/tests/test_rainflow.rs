use crate::MathError;
use crate::core::{make_cycle, reservation_error};
use crate::{RAINFLOW_MAX_INPUT_LEN, RainflowCycle, rainflow_cycles};

type CycleTuple = (f64, f64, f64, usize, usize);

fn tuples(values: &[f64]) -> Vec<CycleTuple> {
    rainflow_cycles(values)
        .unwrap()
        .iter()
        .map(|c| (c.range, c.mean, c.count, c.start_index, c.end_index))
        .collect()
}

fn close(actual: f64, expected: f64) {
    assert!(actual.is_finite());
    let error = (actual - expected).abs();
    assert!(
        error <= 1e-12 || error <= 1e-12 * expected.abs().max(1.0),
        "{actual} != {expected}"
    );
}

fn invariants(values: &[f64], cycles: &[RainflowCycle]) {
    assert!(cycles.len() < values.len());
    let mut counted = 0.0;
    for c in cycles {
        assert!(c.range.is_finite() && c.range > 0.0 && c.mean.is_finite());
        assert!(c.count == 0.5 || c.count == 1.0);
        assert!(c.start_index < c.end_index && c.end_index < values.len());
        assert_eq!(c.range, (values[c.end_index] - values[c.start_index]).abs());
        close(c.mean, (values[c.start_index] + values[c.end_index]) * 0.5);
        counted += 2.0 * c.count * c.range;
    }
    // This helper is used only with small integer/dyadic data (exact arithmetic).
    assert_eq!(
        counted,
        values.windows(2).map(|p| (p[1] - p[0]).abs()).sum::<f64>()
    );
}

#[test]
fn test_two_points_and_monotone_records() {
    assert_eq!(tuples(&[0.0, 2.0]), vec![(2.0, 1.0, 0.5, 0, 1)]);
    assert_eq!(tuples(&[2.0, 0.0]), vec![(2.0, 1.0, 0.5, 0, 1)]);
    assert_eq!(tuples(&[-2.0, -1.0, 0.0, 2.0]), vec![(4.0, 0.0, 0.5, 0, 3)]);
    assert_eq!(tuples(&[2.0, 0.0, -1.0, -2.0]), vec![(4.0, 0.0, 0.5, 0, 3)]);
}

#[test]
fn test_constant_records_have_no_cycles() {
    for n in [2, 3, RAINFLOW_MAX_INPUT_LEN] {
        assert!(tuples(&vec![2.0; n]).is_empty());
    }
    assert!(tuples(&[-0.0, 0.0, -0.0]).is_empty());
}

#[test]
fn test_triangle_and_equal_ranges_are_unmerged_half_cycles() {
    assert_eq!(
        tuples(&[0.0, 1.0, 0.0]),
        vec![(1.0, 0.5, 0.5, 0, 1), (1.0, 0.5, 0.5, 1, 2)]
    );
    assert_eq!(tuples(&[0.0, 1.0, 0.0, 1.0]).len(), 3);
}

#[test]
fn test_nested_full_cycle_precedes_boundary_residual() {
    assert_eq!(
        tuples(&[0.0, 3.0, 1.0, 4.0]),
        vec![(2.0, 2.0, 1.0, 1, 2), (4.0, 2.0, 0.5, 0, 3)]
    );
}

#[test]
fn test_plateaus_preserve_original_endpoint_indices() {
    assert_eq!(
        tuples(&[0.0, 1.0, 1.0, 0.0]),
        vec![(1.0, 0.5, 0.5, 0, 2), (1.0, 0.5, 0.5, 2, 3)]
    );
    assert_eq!(tuples(&[0.0, 0.0, 1.0, 1.0]), vec![(1.0, 0.5, 0.5, 0, 3)]);
    assert_eq!(
        tuples(&[0.0, 0.0, 3.0, 3.0, 1.0, 1.0, 4.0, 4.0]),
        vec![(2.0, 2.0, 1.0, 3, 5), (4.0, 2.0, 0.5, 0, 7)]
    );
}

#[test]
fn test_pinned_independent_reference_fixture() {
    // Numeric oracle: iamlikeme/rainflow, commit 5fc6822fd1c626739c6f95d9b4595ee80330b4fc,
    // tests/test_rainflow.py, TEST_CASE_1. Provenance/hash recorded in SIGNAL_SPEC RF-1.1.
    let values = [-2.0, 1.0, -3.0, 5.0, -1.0, 3.0, -4.0, 4.0, -2.0];
    assert_eq!(
        tuples(&values),
        vec![
            (3.0, -0.5, 0.5, 0, 1),
            (4.0, -1.0, 0.5, 1, 2),
            (4.0, 1.0, 1.0, 4, 5),
            (8.0, 1.0, 0.5, 2, 3),
            (9.0, 0.5, 0.5, 3, 6),
            (8.0, 0.0, 0.5, 6, 7),
            (6.0, 1.0, 0.5, 7, 8),
        ]
    );
}

#[test]
fn test_smallest_subnormals_and_midpoint_underflow() {
    let tiny = f64::from_bits(1);
    assert_eq!(
        tuples(&[tiny, 3.0 * tiny]),
        vec![(2.0 * tiny, 2.0 * tiny, 0.5, 0, 1)]
    );
    let c = rainflow_cycles(&[-tiny, 0.0]).unwrap()[0];
    assert_eq!(c.range, tiny);
    assert_eq!(c.mean.to_bits(), 0.0_f64.to_bits());
}

#[test]
fn test_large_positive_finite_midpoint() {
    let c = rainflow_cycles(&[f64::MAX * 0.5, f64::MAX]).unwrap()[0];
    assert_eq!(c.range, f64::MAX * 0.5);
    close(c.mean, f64::MAX * 0.75);
}

#[test]
fn test_large_negative_finite_midpoint() {
    let c = rainflow_cycles(&[-f64::MAX, -f64::MAX * 0.5]).unwrap()[0];
    assert_eq!(c.range, f64::MAX * 0.5);
    close(c.mean, -f64::MAX * 0.75);
}

#[test]
fn test_large_offset_small_exact_variation() {
    let base = 2.0_f64.powi(50);
    assert_eq!(
        tuples(&[base, base + 8.0, base + 4.0, base + 16.0]),
        vec![(4.0, base + 6.0, 1.0, 1, 2), (16.0, base + 8.0, 0.5, 0, 3)]
    );
}

#[test]
fn test_tiny_excursion_is_not_epsilon_merged() {
    assert_eq!(
        tuples(&[0.0, 1.0, 1.0 - f64::EPSILON]),
        vec![
            (1.0, 0.5, 0.5, 0, 1),
            (f64::EPSILON, 1.0 - f64::EPSILON * 0.5, 0.5, 1, 2),
        ]
    );
}

#[test]
fn test_opposite_sign_midpoint_cancellation() {
    let c = rainflow_cycles(&[-f64::MAX * 0.25, f64::MAX * 0.25]).unwrap()[0];
    assert_eq!(c.range, f64::MAX * 0.5);
    assert_eq!(c.mean.to_bits(), 0.0_f64.to_bits());
}

#[test]
fn test_short_length_failure_precedes_finiteness() {
    for values in [vec![], vec![0.0], vec![f64::NAN]] {
        assert_eq!(
            rainflow_cycles(&values).unwrap_err(),
            MathError::InsufficientDataAlgo {
                required: 2,
                actual: values.len()
            }
        );
    }
}

#[test]
fn test_length_bound_failure_precedes_finiteness() {
    let good = vec![0.0; RAINFLOW_MAX_INPUT_LEN + 1];
    let bad = vec![f64::NAN; RAINFLOW_MAX_INPUT_LEN + 1];
    assert!(matches!(
        rainflow_cycles(&good),
        Err(MathError::InvalidData(_))
    ));
    assert_eq!(
        rainflow_cycles(&good).unwrap_err(),
        rainflow_cycles(&bad).unwrap_err()
    );
}

#[test]
fn test_nonfinite_samples_are_never_skipped() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        for original in [[1.0; 4], [0.0, 3.0, 1.0, 4.0]] {
            for index in 0..4 {
                let mut values = original;
                values[index] = bad;
                assert!(matches!(
                    rainflow_cycles(&values),
                    Err(MathError::InvalidData(_))
                ));
            }
        }
        let mut values = vec![1.0; RAINFLOW_MAX_INPUT_LEN];
        values[RAINFLOW_MAX_INPUT_LEN - 1] = bad;
        assert!(matches!(
            rainflow_cycles(&values),
            Err(MathError::InvalidData(_))
        ));
    }
}

#[test]
fn test_overflow_in_output_or_comparison_is_rejected() {
    for values in [
        vec![-f64::MAX, f64::MAX],
        vec![-f64::MAX, 0.0, f64::MAX],
        vec![0.0, f64::MAX, -f64::MAX],
    ] {
        let err = rainflow_cycles(&values).unwrap_err();
        assert!(
            matches!(err, MathError::NumericalError { operation: Some(ref op), .. } if op == "rainflow_cycles")
        );
    }
}

#[test]
fn test_reservation_failure_conversion_without_oom() {
    let err = Vec::<u64>::new().try_reserve_exact(usize::MAX).unwrap_err();
    assert_eq!(
        reservation_error(err),
        MathError::CalculationError("rainflow_cycles: buffer reservation failed".into())
    );
}

#[test]
fn test_defensive_mean_guard() {
    // Deliberately outside the private helper's validated-finite-input precondition.
    assert!(matches!(
        make_cycle(&[f64::NAN, f64::NAN], 0, 1, 0.5, 1.0),
        Err(MathError::NumericalError { .. })
    ));
}

#[test]
fn test_determinism_twenty_repetitions() {
    let input = [-2.0, 1.0, -3.0, 5.0, -1.0, 3.0, -4.0, 4.0, -2.0];
    let expected = tuples(&input);
    for _ in 0..20 {
        assert_eq!(tuples(&input), expected);
    }
}

#[test]
fn test_deterministic_error_variants_twenty_repetitions() {
    for input in [vec![], vec![0.0, f64::NAN], vec![-f64::MAX, f64::MAX]] {
        let expected = rainflow_cycles(&input).unwrap_err();
        for _ in 0..20 {
            assert_eq!(rainflow_cycles(&input).unwrap_err(), expected);
        }
    }
}

#[test]
fn test_no_call_history_or_input_mutation() {
    let input = [0.0, 3.0, 1.0, 4.0];
    let original = input;
    for _ in 0..20 {
        let _ = rainflow_cycles(&[f64::NAN, 0.0]);
        assert!(tuples(&[1.0, 1.0]).is_empty());
        assert_eq!(
            tuples(&input),
            vec![(2.0, 2.0, 1.0, 1, 2), (4.0, 2.0, 0.5, 0, 3)]
        );
    }
    assert_eq!(input, original);
}

#[test]
fn test_exhaustive_small_integer_invariants() {
    for n in 2..=6 {
        for mut code in 0..5_usize.pow(n) {
            let mut values = vec![0.0; n as usize];
            for x in &mut values {
                *x = (code % 5) as f64 - 2.0;
                code /= 5;
            }
            invariants(&values, &rainflow_cycles(&values).unwrap());
        }
    }
}

#[test]
fn test_exact_affine_and_sign_transformations() {
    let input = [-2.0, 1.0, -3.0, 5.0, -1.0, 3.0, -4.0, 4.0, -2.0];
    let cycles = rainflow_cycles(&input).unwrap();
    for (scale, shift) in [(2.0, 16.0), (-1.0, 0.0)] {
        let transformed: Vec<_> = input.iter().map(|x| scale * x + shift).collect();
        let changed = rainflow_cycles(&transformed).unwrap();
        assert_eq!(changed.len(), cycles.len());
        for (a, b) in cycles.iter().zip(&changed) {
            assert_eq!(
                (a.start_index, a.end_index, a.count),
                (b.start_index, b.end_index, b.count)
            );
            assert_eq!(b.range, scale.abs() * a.range);
            assert_eq!(b.mean, scale * a.mean + shift);
        }
    }
}

#[test]
fn test_maximum_residual_stack_and_closure_cascade() {
    let mut values = vec![0.0; RAINFLOW_MAX_INPUT_LEN];
    for i in 1..values.len() {
        let step = (values.len() - i) as f64;
        values[i] = values[i - 1] + if i % 2 == 1 { step } else { -step };
    }
    let cycles = rainflow_cycles(&values).unwrap();
    assert_eq!(cycles.len(), values.len() - 1);
    assert!(cycles.iter().all(|c| c.count == 0.5));
    invariants(&values, &cycles);
    let last = values.len() - 1;
    values[last] = 2.0 * values.len() as f64;
    invariants(&values, &rainflow_cycles(&values).unwrap());
}

#[test]
fn test_valid_calls_do_not_panic() {
    for values in [vec![0.0, 3.0, 1.0, 4.0], vec![0.0; RAINFLOW_MAX_INPUT_LEN]] {
        assert!(
            std::panic::catch_unwind(|| rainflow_cycles(&values))
                .unwrap()
                .is_ok()
        );
    }
}

#[test]
fn test_invalid_calls_do_not_panic() {
    for values in [
        vec![],
        vec![f64::NAN; 2],
        vec![-f64::MAX, f64::MAX],
        vec![0.0; RAINFLOW_MAX_INPUT_LEN + 1],
    ] {
        assert!(
            std::panic::catch_unwind(|| rainflow_cycles(&values))
                .unwrap()
                .is_err()
        );
    }
}
