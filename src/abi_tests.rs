use super::*;

#[test]
fn shared_tone_matches_baseline_bits() {
    let api = unsafe { &*rptadv_tone_descriptor() };
    for text in ["1100/1,1100/1", "1000+1100@-6/1", "silence/1,1000/1"] {
        for block in [256, 7] {
            let mut baseline = ToneSequence::new(text, -20).unwrap();
            let mut shared = ptr::null_mut();
            assert_eq!(
                unsafe { (api.sequence_create)(text.as_ptr(), text.len(), -20, &mut shared) },
                OK
            );
            let mut total = 0;
            assert_eq!(
                unsafe { (api.sequence_rendered_samples)(shared, &mut total) },
                OK
            );
            assert_eq!(total, baseline.rendered_samples());
            loop {
                let mut expected = vec![99.0; block];
                let expected_written = baseline.render(&mut expected);
                let mut actual = vec![99.0; block];
                let mut actual_written = usize::MAX;
                assert_eq!(
                    unsafe {
                        (api.sequence_render)(
                            shared,
                            actual.as_mut_ptr(),
                            actual.len(),
                            &mut actual_written,
                        )
                    },
                    OK
                );
                assert_eq!(actual_written, expected_written, "{text}: block={block}");
                assert_eq!(
                    actual[..actual_written]
                        .iter()
                        .map(|sample| sample.to_bits())
                        .collect::<Vec<_>>(),
                    expected[..expected_written]
                        .iter()
                        .map(|sample| sample.to_bits())
                        .collect::<Vec<_>>(),
                    "{text}: block={block}"
                );
                if actual_written == 0 {
                    break;
                }
            }
            unsafe { (api.sequence_destroy)(shared) };
        }
    }
}

#[test]
fn oscillator_preserves_morse_waveform_and_reset() {
    let api = unsafe { &*rptadv_tone_descriptor() };
    let mut oscillator = ptr::null_mut();
    assert_eq!(
        unsafe { (api.oscillator_create)(1100.0, 0.1, &mut oscillator) },
        OK
    );
    let mut samples = [0.0; 32];
    assert_eq!(
        unsafe { (api.oscillator_render)(oscillator, samples.as_mut_ptr(), samples.len()) },
        OK
    );
    let mut phase = 0.0_f32;
    let step = 1100.0_f32 / 48_000.0;
    for sample in samples {
        assert_eq!(
            sample.to_bits(),
            (0.1_f32 * (core::f32::consts::TAU * phase).sin()).to_bits()
        );
        phase = (phase + step).fract();
    }
    assert_eq!(unsafe { (api.oscillator_reset)(oscillator) }, OK);
    let mut first = 99.0;
    assert_eq!(
        unsafe { (api.oscillator_render)(oscillator, &mut first, 1) },
        OK
    );
    assert_eq!(first.to_bits(), 0);
    assert_eq!(
        unsafe { (api.oscillator_render)(oscillator, ptr::null_mut(), 0) },
        OK
    );
    unsafe { (api.oscillator_destroy)(oscillator) };
}

#[test]
fn abi_rejects_bad_inputs_without_exposing_handles() {
    let api = unsafe { &*rptadv_tone_descriptor() };
    let mut sequence = ptr::null_mut();
    let text = b"1000/1";
    assert_eq!(
        unsafe { (api.sequence_create)(text.as_ptr(), text.len(), -20, ptr::null_mut()) },
        INVALID_ARGUMENT
    );
    assert_eq!(
        unsafe { (api.sequence_create)(ptr::null(), 0, -20, &mut sequence) },
        INVALID_SEQUENCE
    );
    assert!(sequence.is_null());
    assert_eq!(
        unsafe { (api.sequence_create)(ptr::null(), 1, -20, &mut sequence) },
        INVALID_ARGUMENT
    );
    assert_eq!(
        unsafe { (api.sequence_create)(b"\xff".as_ptr(), 1, -20, &mut sequence) },
        INVALID_ARGUMENT
    );
    assert_eq!(
        unsafe { (api.sequence_create)(text.as_ptr(), text.len(), -20, &mut sequence) },
        OK
    );
    let mut total = 0;
    assert_eq!(
        unsafe { (api.sequence_rendered_samples)(ptr::null(), &mut total) },
        INVALID_ARGUMENT
    );
    assert_eq!(
        unsafe { (api.sequence_rendered_samples)(sequence, ptr::null_mut()) },
        INVALID_ARGUMENT
    );
    let mut written = usize::MAX;
    assert_eq!(
        unsafe { (api.sequence_render)(sequence, ptr::null_mut(), 0, ptr::null_mut()) },
        INVALID_ARGUMENT
    );
    assert_eq!(
        unsafe { (api.sequence_render)(ptr::null_mut(), ptr::null_mut(), 0, &mut written) },
        INVALID_ARGUMENT
    );
    assert_eq!(written, 0);
    assert_eq!(
        unsafe { (api.sequence_render)(sequence, ptr::null_mut(), 1, &mut written) },
        INVALID_ARGUMENT
    );
    assert_eq!(
        unsafe { (api.sequence_render)(sequence, ptr::null_mut(), 0, &mut written) },
        OK
    );
    assert_eq!(written, 0);
    unsafe {
        (api.sequence_destroy)(sequence);
        (api.sequence_destroy)(ptr::null_mut());
    }

    let mut oscillator = ptr::null_mut();
    assert_eq!(
        unsafe { (api.oscillator_create)(1100.0, 0.1, ptr::null_mut()) },
        INVALID_ARGUMENT
    );
    for (frequency, amplitude) in [
        (f32::NAN, 0.1),
        (0.0, 0.1),
        (24_000.0, 0.1),
        (1100.0, f32::NAN),
        (1100.0, -0.1),
    ] {
        assert_eq!(
            unsafe { (api.oscillator_create)(frequency, amplitude, &mut oscillator) },
            INVALID_ARGUMENT
        );
        assert!(oscillator.is_null());
    }
    assert_eq!(
        unsafe { (api.oscillator_create)(1100.0, 0.1, &mut oscillator) },
        OK
    );
    assert_eq!(
        unsafe { (api.oscillator_reset)(ptr::null_mut()) },
        INVALID_ARGUMENT
    );
    assert_eq!(
        unsafe { (api.oscillator_render)(ptr::null_mut(), ptr::null_mut(), 0) },
        INVALID_ARGUMENT
    );
    assert_eq!(
        unsafe { (api.oscillator_render)(oscillator, ptr::null_mut(), 1) },
        INVALID_ARGUMENT
    );
    unsafe {
        (api.oscillator_destroy)(oscillator);
        (api.oscillator_destroy)(ptr::null_mut());
    }
}
