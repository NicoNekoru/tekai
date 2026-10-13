//! The independent reference expands one logical byte at a time. The retained
//! upstream kernel additionally writes padding, so only its published prefix
//! is compared. The optimized kernel must leave history and padding untouched.

use super::{
    copy_short_distance_match, decompress_to_vec, DecompressionError, Decompressor, State,
};
use proptest::prelude::*;

fn logical_match(output: &mut [u8], output_index: usize, length: usize, distance: usize) {
    for index in 0..length {
        output[output_index + index] = output[output_index + index - distance];
    }
}

fn original_full_room_match(
    output: &mut [u8],
    output_index: usize,
    length: usize,
    distance: usize,
) -> usize {
    let start = output_index - distance;
    output.copy_within(start..start + 16, output_index);
    let mut copies = 1;
    if length > 16 || distance < 16 {
        for index in (0..length).step_by(distance.min(16)).skip(1) {
            output.copy_within(start + index..start + index + 16, output_index + index);
            copies += 1;
        }
    }
    copies
}

fn prefix_byte(index: usize, variant: usize) -> u8 {
    match variant {
        0 => 0,
        1 => 255,
        2 => (index % 2) as u8 * 128,
        3 => index as u8,
        _ => (index as u8).wrapping_mul(37).wrapping_add(193),
    }
}

#[test]
fn every_short_distance_and_length_matches_both_references_with_exact_work() {
    for distance in 2..16 {
        for length in 3..=258 {
            for prefix_padding in [0, 1, 7, 16, 31] {
                for variant in 0..5 {
                    let output_index = distance + prefix_padding;
                    let mut original = vec![0xD3; output_index + length + 15 + 11];
                    for (index, byte) in original[..output_index].iter_mut().enumerate() {
                        *byte = prefix_byte(index, variant);
                    }
                    let mut expected = original.clone();
                    logical_match(&mut expected, output_index, length, distance);
                    let mut upstream = original.clone();
                    let old_copies =
                        original_full_room_match(&mut upstream, output_index, length, distance);
                    let mut actual = original.clone();
                    let work =
                        copy_short_distance_match(&mut actual, output_index, length, distance);

                    assert_eq!(actual, expected, "distance={distance}, length={length}, prefix={output_index}, variant={variant}");
                    assert_eq!(
                        actual[..output_index + length],
                        upstream[..output_index + length]
                    );
                    assert_eq!(actual[..output_index], original[..output_index]);
                    assert_eq!(
                        actual[output_index + length..],
                        original[output_index + length..]
                    );
                    assert_eq!(work.bytes, length);
                    assert!(work.copies <= 9, "{work:?}");
                    assert!(work.copies <= old_copies, "{work:?}, old={old_copies}");
                }
            }
        }
    }
}

#[test]
fn maximum_matches_remove_redundant_short_distance_writes() {
    for (distance, expected_copies, old_copies) in [(2, 9, 129), (4, 8, 65), (15, 6, 18)] {
        let mut output = vec![0xA7; distance + 258 + 15];
        for (index, byte) in output[..distance].iter_mut().enumerate() {
            *byte = prefix_byte(index, 4);
        }
        let mut upstream = output.clone();
        assert_eq!(
            original_full_room_match(&mut upstream, distance, 258, distance),
            old_copies
        );
        let work = copy_short_distance_match(&mut output, distance, 258, distance);
        assert_eq!(work.copies, expected_copies);
        assert_eq!(work.bytes, 258);
        assert_eq!(output[..distance + 258], upstream[..distance + 258]);
        assert_eq!(output[distance + 258..], [0xA7; 15]);
    }
}

#[derive(Clone, Copy, Default)]
struct BoundaryProbes {
    zero_space_backref: bool,
    zero_space_rle: bool,
    footer_after_payload: bool,
}

struct StreamingResult {
    data: Vec<u8>,
    pending_backref: bool,
    pending_rle: bool,
    boundaries: BoundaryProbes,
}

struct StreamingFailure {
    error: DecompressionError,
    pending_backref: bool,
    pending_rle: bool,
    boundaries: BoundaryProbes,
}

/// Retain all history while exposing fragmented compressed input and gradually
/// growing initialized output storage. This keeps checksum checking enabled.
fn decode_fragmented_growing(
    input: &[u8],
    expected_bytes: usize,
    fragments: &[usize],
    initial_output: usize,
    growth: usize,
    probe_boundaries: bool,
) -> Result<StreamingResult, StreamingFailure> {
    assert!(!fragments.is_empty() && fragments.iter().all(|&length| length > 0));
    assert!(initial_output > 0 && growth > 0);
    assert!(!probe_boundaries || input.len() > 1);
    let mut decoder = Decompressor::new();
    assert!(!decoder.ignore_adler32);
    let mut output = vec![0; initial_output.min(expected_bytes + 15)];
    let mut input_position = 0;
    let mut input_end = 0;
    let mut output_position = 0;
    let mut pending_backref = false;
    let mut pending_rle = false;
    let mut boundaries = BoundaryProbes::default();
    let input_limit = input.len() - usize::from(probe_boundaries);
    let maximum_iterations = input.len() + expected_bytes * 2 + 1024;
    for iteration in 0..maximum_iterations {
        if decoder.is_done() {
            break;
        }
        if probe_boundaries && output_position == expected_bytes && input_end == input_limit {
            // The final checksum byte has never been offered. No payload or
            // pending match remains, so finishing must need no output space.
            assert_eq!(input_position, input_limit);
            assert_eq!(decoder.state, State::Checksum);
            assert!(decoder.queued_backref.is_none() && decoder.queued_rle.is_none());
            let checksum_before = decoder.checksum.finish();
            let progress = decoder
                .read(&[], &mut output[..output_position], output_position, false)
                .map_err(|error| StreamingFailure {
                    error,
                    pending_backref,
                    pending_rle,
                    boundaries,
                })?;
            assert_eq!(progress, (0, 0));
            assert_eq!(decoder.state, State::Checksum);
            assert_eq!(decoder.checksum.finish(), checksum_before);
            boundaries.footer_after_payload = true;
            let progress = decoder
                .read(
                    &input[input_position..],
                    &mut output[..output_position],
                    output_position,
                    true,
                )
                .map_err(|error| StreamingFailure {
                    error,
                    pending_backref,
                    pending_rle,
                    boundaries,
                })?;
            assert_eq!(progress, (1, 0));
            assert!(decoder.is_done());
            input_position += progress.0;
            input_end = input.len();
            continue;
        }
        if output_position == output.len() {
            if probe_boundaries
                && (decoder.queued_backref.is_some() || decoder.queued_rle.is_some())
            {
                // A real pending match must survive a read with no writable
                // output bytes, without consuming input or changing checksum.
                let backref_before = decoder.queued_backref;
                let rle_before = decoder.queued_rle;
                let checksum_before = decoder.checksum.finish();
                let output_before = output.clone();
                let progress = decoder
                    .read(
                        &input[input_position..input_end],
                        &mut output,
                        output_position,
                        false,
                    )
                    .map_err(|error| StreamingFailure {
                        error,
                        pending_backref,
                        pending_rle,
                        boundaries,
                    })?;
                assert_eq!(progress, (0, 0));
                assert_eq!(decoder.queued_backref, backref_before);
                assert_eq!(decoder.queued_rle, rle_before);
                assert_eq!(decoder.checksum.finish(), checksum_before);
                assert_eq!(output, output_before);
                boundaries.zero_space_backref |= backref_before.is_some();
                boundaries.zero_space_rle |= rle_before.is_some();
            }
            let new_length = (output.len() + growth).min(expected_bytes + 15);
            assert!(
                new_length > output.len(),
                "valid stream exceeded its expected output bound"
            );
            output.resize(new_length, 0);
        }
        input_end = (input_end + fragments[iteration % fragments.len()]).min(input_limit);
        let (consumed, produced) = match decoder.read(
            &input[input_position..input_end],
            &mut output,
            output_position,
            input_end == input.len(),
        ) {
            Ok(progress) => progress,
            Err(error) => {
                return Err(StreamingFailure {
                    error,
                    pending_backref,
                    pending_rle,
                    boundaries,
                })
            }
        };
        assert!(consumed <= input_end - input_position);
        assert!(produced <= output.len() - output_position);
        input_position += consumed;
        output_position += produced;
        assert!(output_position <= expected_bytes);
        pending_backref |= decoder.queued_backref.is_some();
        pending_rle |= decoder.queued_rle.is_some();
    }
    assert!(
        decoder.is_done(),
        "bounded streaming decoder did not finish"
    );
    assert_eq!(input_position, input.len());
    assert_eq!(output_position, expected_bytes);
    output.truncate(output_position);
    Ok(StreamingResult {
        data: output,
        pending_backref,
        pending_rle,
        boundaries,
    })
}

#[test]
fn fragmented_streams_retain_pending_short_matches_across_output_growth() {
    for distance in 2..16 {
        let period: Vec<_> = (0..distance).map(|index| prefix_byte(index, 4)).collect();
        let payload = period.repeat(512);
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&payload, 3);
        let decoded =
            decode_fragmented_growing(&compressed, payload.len(), &[1, 2, 7, 8, 31], 1, 7, false)
                .unwrap_or_else(|failure| panic!("valid independent stream: {:?}", failure.error));
        assert_eq!(decoded.data, payload);
        assert!(
            decoded.pending_backref,
            "distance={distance} never retained a pending match"
        );
    }
}

#[test]
fn partial_queued_backrefs_account_for_every_published_checksum_byte() {
    // The retained proptest seed shrank to this exact valid alternating stream.
    let payload = [0_u8, 1].repeat(256);
    let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&payload, 3);
    assert_eq!(decompress_to_vec(&compressed).unwrap(), payload);
    let decoded = decode_fragmented_growing(&compressed, payload.len(), &[1], 1, 1, true)
        .unwrap_or_else(|failure| panic!("valid alternating stream: {:?}", failure.error));
    assert_eq!(decoded.data, payload);
    assert!(decoded.pending_backref);
    assert!(decoded.boundaries.zero_space_backref);
    assert!(decoded.boundaries.footer_after_payload);

    let mut corrupted = compressed;
    *corrupted.last_mut().unwrap() ^= 1;
    let failure = match decode_fragmented_growing(&corrupted, payload.len(), &[1], 1, 1, true) {
        Ok(_) => panic!("corrupt backref checksum was accepted"),
        Err(failure) => failure,
    };
    assert_eq!(failure.error, DecompressionError::WrongChecksum);
    assert!(failure.pending_backref);
    assert!(failure.boundaries.zero_space_backref);
    assert!(failure.boundaries.footer_after_payload);
}

#[test]
fn partial_queued_rle_accounts_for_every_published_checksum_byte() {
    let payload = vec![0xA5; 512];
    let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&payload, 3);
    assert_eq!(decompress_to_vec(&compressed).unwrap(), payload);
    let decoded = decode_fragmented_growing(&compressed, payload.len(), &[1], 1, 1, true)
        .unwrap_or_else(|failure| panic!("valid RLE stream: {:?}", failure.error));
    assert_eq!(decoded.data, payload);
    assert!(decoded.pending_rle);
    assert!(decoded.boundaries.zero_space_rle);
    assert!(decoded.boundaries.footer_after_payload);

    let mut corrupted = compressed;
    *corrupted.last_mut().unwrap() ^= 1;
    let failure = match decode_fragmented_growing(&corrupted, payload.len(), &[1], 1, 1, true) {
        Ok(_) => panic!("corrupt RLE checksum was accepted"),
        Err(failure) => failure,
    };
    assert_eq!(failure.error, DecompressionError::WrongChecksum);
    assert!(failure.pending_rle);
    assert!(failure.boundaries.zero_space_rle);
    assert!(failure.boundaries.footer_after_payload);
}

#[test]
fn full_input_decode_exercises_both_actual_short_match_dispatch_sites() {
    let mut payload = [0x90_u8, 0x14, 0xAB, 0xFF].repeat(2048);
    let mut separator_state = 0x735A_2D91_u32;
    for _ in 0..1024 {
        separator_state ^= separator_state << 13;
        separator_state ^= separator_state >> 17;
        separator_state ^= separator_state << 5;
        payload.push(separator_state as u8);
    }
    payload.extend_from_slice(&[0x02_u8, 0x80, 0x11].repeat(1024));
    let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&payload, 3);
    assert_eq!(
        miniz_oxide::inflate::decompress_to_vec_zlib(&compressed).unwrap(),
        payload,
    );
    let mut decoder = Decompressor::new();
    // Include exactly the unchanged full-room branch's 15-byte padding.
    let mut output = vec![0xD3; payload.len() + 15];
    let (consumed, produced) = decoder.read(&compressed, &mut output, 0, true).unwrap();
    assert!(decoder.is_done());
    assert_eq!(consumed, compressed.len());
    assert_eq!(produced, payload.len());
    assert_eq!(output[..produced], payload);
    assert!(
        decoder.short_match_dispatches[0] > 0,
        "fast-loop dispatch was not exercised"
    );
    assert!(
        decoder.short_match_dispatches[1] > 0,
        "careful-loop dispatch was not exercised"
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn independent_periodic_streams_match_under_fragmentation_and_output_growth(
        mut period in proptest::collection::vec(any::<u8>(), 2..16),
        distinct in 1_u8..=255,
        repetitions in 256_usize..=512,
        prefix in proptest::collection::vec(any::<u8>(), 0..48),
        fragments in proptest::collection::vec(1_usize..=32, 1..9),
        initial_output in 1_usize..=17,
        growth in 1_usize..=31,
    ) {
        // Keep a genuinely multi-byte period rather than only distance-one RLE.
        period[1] = period[0] ^ distinct;
        let mut payload = prefix;
        payload.extend_from_slice(&period.repeat(repetitions));
        prop_assert!(payload.len() <= 48 + 15 * 512);
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&payload, 3);
        let whole = decompress_to_vec(&compressed).expect("valid independent miniz_oxide stream");
        prop_assert_eq!(&whole, &payload);
        let streamed = decode_fragmented_growing(
            &compressed,
            payload.len(),
            &fragments,
            initial_output,
            growth,
            false,
        ).unwrap_or_else(|failure| panic!("valid independent stream: {:?}", failure.error));
        prop_assert_eq!(&streamed.data, &payload);
        prop_assert!(streamed.pending_backref || streamed.pending_rle);
    }
}
