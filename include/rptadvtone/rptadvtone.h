/**
 * @file rptadvtone.h
 * @brief ABI 1 for 48 kHz tone sequences and a reusable sine oscillator.
 */
#ifndef RPTADVTONE_H
#define RPTADVTONE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/** ABI major carried by the descriptor and SONAME. */
#define RPTADV_TONE_ABI_VERSION 1U
/** Successful operation. */
#define RPTADV_TONE_OK 0
/** Null, nonfinite, or otherwise invalid argument. */
#define RPTADV_TONE_INVALID_ARGUMENT 1
/** Rejected tone sequence grammar or bounds. */
#define RPTADV_TONE_INVALID_SEQUENCE 2

/** Opaque, setup-allocated tone sequence. */
struct rptadv_tone_sequence;
/** Opaque, setup-allocated oscillator for Morse's waveform primitive. */
struct rptadv_tone_oscillator;

/** Immutable ABI 1 entrypoint table. Resolve and validate during setup. */
struct rptadv_tone_descriptor {
	/** Size of this structure in bytes. */
	uint32_t struct_size;
	/** Major ABI version. */
	uint32_t abi_version;
	/** NUL-terminated identity, "rptadv.tone". */
	const char *capability_name;
	/** Parse UTF-8 bytes and allocate a bounded sequence. */
	int32_t (*sequence_create)(const uint8_t *text, size_t text_len,
				   int8_t default_level_db,
				   struct rptadv_tone_sequence **out);
	/** Read the total 48 kHz sample count. */
	int32_t (*sequence_rendered_samples)(const struct rptadv_tone_sequence *sequence,
					     size_t *out);
	/** Render the next samples and report exactly how many were written. */
	int32_t (*sequence_render)(struct rptadv_tone_sequence *sequence,
				   float *output, size_t capacity, size_t *written);
	/** Release a sequence after its callback users stop; NULL is accepted. */
	void (*sequence_destroy)(struct rptadv_tone_sequence *sequence);
	/** Allocate the same sine/phase primitive used by Morse. */
	int32_t (*oscillator_create)(float frequency_hz, float amplitude,
				     struct rptadv_tone_oscillator **out);
	/** Restart phase at zero. */
	int32_t (*oscillator_reset)(struct rptadv_tone_oscillator *oscillator);
	/** Fill exactly count samples; a NULL output is valid when count is zero. */
	int32_t (*oscillator_render)(struct rptadv_tone_oscillator *oscillator,
				     float *output, size_t count);
	/** Release an oscillator after its callback users stop; NULL is accepted. */
	void (*oscillator_destroy)(struct rptadv_tone_oscillator *oscillator);
};

/** Return the immutable descriptor; it remains valid for the library lifetime. */
const struct rptadv_tone_descriptor *rptadv_tone_descriptor(void);

#ifdef __cplusplus
}
#endif
#endif
