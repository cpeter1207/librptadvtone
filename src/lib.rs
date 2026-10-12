//! Stable C ABI for the existing 48 kHz tone renderer.

mod oscillator;
mod tone;

use core::ffi::c_char;
use core::mem::size_of;
use core::ptr;

use oscillator::Oscillator;
use tone::ToneSequence;

const OK: i32 = 0;
const INVALID_ARGUMENT: i32 = 1;
const INVALID_SEQUENCE: i32 = 2;

/// Opaque owner of one parsed tone sequence.
#[repr(C)]
pub struct ToneSequenceHandle(ToneSequence);

/// Opaque owner of one sine oscillator.
#[repr(C)]
pub struct ToneOscillatorHandle(Oscillator);

/// Immutable ABI 1 operation table.
#[repr(C)]
pub struct ToneDescriptor {
    struct_size: u32,
    abi_version: u32,
    capability_name: *const c_char,
    sequence_create:
        unsafe extern "C" fn(*const u8, usize, i8, *mut *mut ToneSequenceHandle) -> i32,
    sequence_rendered_samples: unsafe extern "C" fn(*const ToneSequenceHandle, *mut usize) -> i32,
    sequence_render:
        unsafe extern "C" fn(*mut ToneSequenceHandle, *mut f32, usize, *mut usize) -> i32,
    sequence_destroy: unsafe extern "C" fn(*mut ToneSequenceHandle),
    oscillator_create: unsafe extern "C" fn(f32, f32, *mut *mut ToneOscillatorHandle) -> i32,
    oscillator_reset: unsafe extern "C" fn(*mut ToneOscillatorHandle) -> i32,
    oscillator_render: unsafe extern "C" fn(*mut ToneOscillatorHandle, *mut f32, usize) -> i32,
    oscillator_destroy: unsafe extern "C" fn(*mut ToneOscillatorHandle),
}

// The descriptor and its NUL-terminated identity never change.
unsafe impl Sync for ToneDescriptor {}

static DESCRIPTOR: ToneDescriptor = ToneDescriptor {
    struct_size: size_of::<ToneDescriptor>() as u32,
    abi_version: 1,
    capability_name: c"rptadv.tone".as_ptr(),
    sequence_create,
    sequence_rendered_samples,
    sequence_render,
    sequence_destroy,
    oscillator_create,
    oscillator_reset,
    oscillator_render,
    oscillator_destroy,
};

/// Return the immutable ABI 1 descriptor for setup-time validation.
#[unsafe(no_mangle)]
pub extern "C" fn rptadv_tone_descriptor() -> *const ToneDescriptor {
    &raw const DESCRIPTOR
}

unsafe extern "C" fn sequence_create(
    text: *const u8,
    text_len: usize,
    default_level_db: i8,
    out: *mut *mut ToneSequenceHandle,
) -> i32 {
    let Some(out) = (unsafe { out.as_mut() }) else {
        return INVALID_ARGUMENT;
    };
    *out = ptr::null_mut();
    let bytes = if text_len == 0 {
        &[][..]
    } else if text.is_null() {
        return INVALID_ARGUMENT;
    } else {
        unsafe { core::slice::from_raw_parts(text, text_len) }
    };
    let Ok(text) = core::str::from_utf8(bytes) else {
        return INVALID_ARGUMENT;
    };
    let Ok(sequence) = ToneSequence::new(text, default_level_db) else {
        return INVALID_SEQUENCE;
    };
    *out = Box::into_raw(Box::new(ToneSequenceHandle(sequence)));
    OK
}

unsafe extern "C" fn sequence_rendered_samples(
    sequence: *const ToneSequenceHandle,
    out: *mut usize,
) -> i32 {
    let (Some(sequence), Some(out)) = (unsafe { sequence.as_ref() }, unsafe { out.as_mut() })
    else {
        return INVALID_ARGUMENT;
    };
    *out = sequence.0.rendered_samples();
    OK
}

unsafe extern "C" fn sequence_render(
    sequence: *mut ToneSequenceHandle,
    output: *mut f32,
    capacity: usize,
    written: *mut usize,
) -> i32 {
    let Some(written) = (unsafe { written.as_mut() }) else {
        return INVALID_ARGUMENT;
    };
    *written = 0;
    let Some(sequence) = (unsafe { sequence.as_mut() }) else {
        return INVALID_ARGUMENT;
    };
    let output = if capacity == 0 {
        &mut [][..]
    } else if output.is_null() {
        return INVALID_ARGUMENT;
    } else {
        unsafe { core::slice::from_raw_parts_mut(output, capacity) }
    };
    *written = sequence.0.render(output);
    OK
}

unsafe extern "C" fn sequence_destroy(sequence: *mut ToneSequenceHandle) {
    if !sequence.is_null() {
        drop(unsafe { Box::from_raw(sequence) });
    }
}

unsafe extern "C" fn oscillator_create(
    frequency_hz: f32,
    amplitude: f32,
    out: *mut *mut ToneOscillatorHandle,
) -> i32 {
    let Some(out) = (unsafe { out.as_mut() }) else {
        return INVALID_ARGUMENT;
    };
    *out = ptr::null_mut();
    if !frequency_hz.is_finite()
        || frequency_hz <= 0.0
        || frequency_hz >= 24_000.0
        || !amplitude.is_finite()
        || amplitude < 0.0
    {
        return INVALID_ARGUMENT;
    }
    *out = Box::into_raw(Box::new(ToneOscillatorHandle(Oscillator::new(
        frequency_hz,
        amplitude,
    ))));
    OK
}

unsafe extern "C" fn oscillator_reset(oscillator: *mut ToneOscillatorHandle) -> i32 {
    let Some(oscillator) = (unsafe { oscillator.as_mut() }) else {
        return INVALID_ARGUMENT;
    };
    oscillator.0.reset();
    OK
}

unsafe extern "C" fn oscillator_render(
    oscillator: *mut ToneOscillatorHandle,
    output: *mut f32,
    count: usize,
) -> i32 {
    let Some(oscillator) = (unsafe { oscillator.as_mut() }) else {
        return INVALID_ARGUMENT;
    };
    let output = if count == 0 {
        &mut [][..]
    } else if output.is_null() {
        return INVALID_ARGUMENT;
    } else {
        unsafe { core::slice::from_raw_parts_mut(output, count) }
    };
    oscillator.0.render(output);
    OK
}

unsafe extern "C" fn oscillator_destroy(oscillator: *mut ToneOscillatorHandle) {
    if !oscillator.is_null() {
        drop(unsafe { Box::from_raw(oscillator) });
    }
}

#[cfg(test)]
#[path = "abi_tests.rs"]
mod tests;
