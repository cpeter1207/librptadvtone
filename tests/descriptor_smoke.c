/** @file descriptor_smoke.c @brief Exercise the installed dynamic tone ABI. */
#include <assert.h>
#include <stdint.h>
#include <string.h>

#include "rptadvtone/rptadvtone.h"

static uint32_t bits(float value)
{
	uint32_t result;
	memcpy(&result, &value, sizeof(result));
	return result;
}

int main(void)
{
	const struct rptadv_tone_descriptor *api = rptadv_tone_descriptor();
	struct rptadv_tone_sequence *sequence = NULL;
	struct rptadv_tone_oscillator *oscillator = NULL;
	const uint8_t text[] = "1100/1,1100/1";
	float whole[96] = { 0 };
	float blocks[96] = { 0 };
	float wave[8] = { 0 };
	size_t total = 0, written = 0, offset = 0;

	assert(api != NULL);
	assert(api->struct_size == sizeof(*api));
	assert(api->abi_version == RPTADV_TONE_ABI_VERSION);
	assert(strcmp(api->capability_name, "rptadv.tone") == 0);
	assert(api->sequence_create && api->sequence_rendered_samples);
	assert(api->sequence_render && api->sequence_destroy);
	assert(api->oscillator_create && api->oscillator_reset);
	assert(api->oscillator_render && api->oscillator_destroy);
	assert(api->sequence_create(text, sizeof(text) - 1, -20, &sequence) == RPTADV_TONE_OK);
	assert(sequence != NULL);
	assert(api->sequence_rendered_samples(sequence, &total) == RPTADV_TONE_OK);
	assert(total == 96);
	assert(api->sequence_render(sequence, whole, 96, &written) == RPTADV_TONE_OK);
	assert(written == 96);
	assert(bits(whole[0]) == 0x00000000U);
	assert(bits(whole[1]) == 0x3c6b192bU);
	assert(api->sequence_render(sequence, NULL, 0, &written) == RPTADV_TONE_OK);
	assert(written == 0);
	api->sequence_destroy(sequence);
	sequence = NULL;
	assert(api->sequence_create(text, sizeof(text) - 1, -20, &sequence) == RPTADV_TONE_OK);
	while (offset < total) {
		size_t capacity = total - offset < 7 ? total - offset : 7;
		assert(api->sequence_render(sequence, blocks + offset, capacity, &written) == RPTADV_TONE_OK);
		assert(written == capacity);
		offset += written;
	}
	for (offset = 0; offset < total; ++offset)
		assert(bits(blocks[offset]) == bits(whole[offset]));
	api->sequence_destroy(sequence);
	assert(api->sequence_create((const uint8_t *)"1000/0", 6, -20, &sequence) == RPTADV_TONE_INVALID_SEQUENCE);
	assert(sequence == NULL);
	assert(api->sequence_create(NULL, 3, -20, &sequence) == RPTADV_TONE_INVALID_ARGUMENT);
	assert(api->oscillator_create(1100.0F, 0.1F, &oscillator) == RPTADV_TONE_OK);
	assert(api->oscillator_render(oscillator, wave, 7) == RPTADV_TONE_OK);
	assert(bits(wave[0]) == 0);
	assert(api->oscillator_reset(oscillator) == RPTADV_TONE_OK);
	assert(api->oscillator_render(oscillator, wave, 7) == RPTADV_TONE_OK);
	assert(bits(wave[0]) == 0);
	assert(api->oscillator_render(oscillator, NULL, 0) == RPTADV_TONE_OK);
	api->oscillator_destroy(oscillator);
	api->oscillator_destroy(NULL);
	return 0;
}
