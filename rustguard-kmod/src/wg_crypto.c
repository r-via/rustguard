// SPDX-License-Identifier: GPL-2.0
/*
 * RustGuard — C shim for kernel crypto.
 *
 * Uses the kernel's chacha20poly1305 buffer API (not SG) for all operations.
 * sg_init_one crashes when data crosses 4KB page boundaries (virt_to_page
 * returns wrong page). The buffer API handles page mapping internally.
 */

#include <linux/module.h>
#include <linux/slab.h>
#include <linux/random.h>
#include <linux/skbuff.h>
#include <crypto/chacha20poly1305.h>
#include <crypto/blake2s.h>
#include <crypto/curve25519.h>
#include <crypto/algapi.h>
#include <linux/ktime.h>

/* ── Prototypes ────────────────────────────────────────────────────── */

int wg_chacha20poly1305_encrypt(
	const u8 key[32], u64 nonce, const u8 *src, u32 src_len,
	const u8 *ad, u32 ad_len, u8 *dst);
int wg_chacha20poly1305_decrypt(
	const u8 key[32], u64 nonce, const u8 *src, u32 src_len,
	const u8 *ad, u32 ad_len, u8 *dst);
int wg_encrypt_skb(struct sk_buff *skb, u32 plaintext_off, u32 plaintext_len,
		   u64 nonce, const u8 key[32]);
struct sk_buff *wg_decrypt_skb_full(struct sk_buff *skb, u32 hdr_len,
				    u64 nonce, const u8 key[32]);
void wg_blake2s_hash(const u8 *const *chunks, const u32 *chunk_lens,
	u32 num_chunks, u8 out[32]);
void wg_blake2s_256_hmac(const u8 key[32], const u8 *data, u32 data_len,
	u8 out[32]);
void wg_blake2s_256_mac(const u8 *key, u32 key_len,
	const u8 *data, u32 data_len, u8 out[32]);
void wg_hkdf(const u8 key[32], const u8 *input, u32 input_len,
	u8 out1[32], u8 out2[32], u8 out3[32]);
int wg_curve25519(u8 out[32], const u8 scalar[32], const u8 point[32]);
void wg_curve25519_generate_secret(u8 secret[32]);
void wg_curve25519_generate_public(u8 pub_key[32], const u8 secret[32]);
void wg_get_random_bytes(u8 *buf, u32 len);
int wg_xchacha20poly1305_encrypt(
	const u8 key[32], const u8 nonce[24], const u8 *src, u32 src_len,
	const u8 *ad, u32 ad_len, u8 *dst);
int wg_xchacha20poly1305_decrypt(
	const u8 key[32], const u8 nonce[24], const u8 *src, u32 src_len,
	const u8 *ad, u32 ad_len, u8 *dst);
u64 wg_ktime_get_ns(void);
void wg_ktime_get_real(s64 *secs, s64 *nsecs);
void wg_memzero(void *ptr, size_t len);
int wg_crypto_memneq(const void *a, const void *b, size_t len);
int wg_crypto_init(void);
void wg_crypto_exit(void);

/* ── ChaCha20-Poly1305 (buffer API) ───────────────────────────────── */

int wg_chacha20poly1305_encrypt(
	const u8 key[32], u64 nonce, const u8 *src, u32 src_len,
	const u8 *ad, u32 ad_len, u8 *dst)
{
	chacha20poly1305_encrypt(dst, src, src_len, ad, ad_len, nonce, key);
	return 0;
}
EXPORT_SYMBOL_GPL(wg_chacha20poly1305_encrypt);

int wg_chacha20poly1305_decrypt(
	const u8 key[32], u64 nonce, const u8 *src, u32 src_len,
	const u8 *ad, u32 ad_len, u8 *dst)
{
	if (src_len < CHACHA20POLY1305_AUTHTAG_SIZE)
		return -EINVAL;
	if (!chacha20poly1305_decrypt(dst, src, src_len, ad, ad_len, nonce, key))
		return -EBADMSG;
	return 0;
}
EXPORT_SYMBOL_GPL(wg_chacha20poly1305_decrypt);

/* ── skb encrypt (TX) ──────────────────────────────────────────────── */

int wg_encrypt_skb(struct sk_buff *skb, u32 plaintext_off, u32 plaintext_len,
		   u64 nonce, const u8 key[32])
{
	u8 *pt = skb->data + plaintext_off;
	chacha20poly1305_encrypt(pt, pt, plaintext_len, NULL, 0, nonce, key);
	return 0;
}
EXPORT_SYMBOL_GPL(wg_encrypt_skb);

/* ── skb decrypt (RX) — full pipeline ──────────────────────────────── */

struct sk_buff *wg_decrypt_skb_full(struct sk_buff *skb, u32 hdr_len,
				    u64 nonce, const u8 key[32])
{
	struct sk_buff *nskb;
	u32 ct_len, pt_len;
	u8 *ct_data;

	if (skb->len <= hdr_len + CHACHA20POLY1305_AUTHTAG_SIZE)
		return NULL;

	ct_len = skb->len - hdr_len;
	pt_len = ct_len - CHACHA20POLY1305_AUTHTAG_SIZE;
	ct_data = skb->data + hdr_len;

	nskb = alloc_skb(pt_len, GFP_ATOMIC);
	if (!nskb)
		return NULL;

	if (!chacha20poly1305_decrypt(skb_put(nskb, pt_len), ct_data,
				      ct_len, NULL, 0, nonce, key)) {
		kfree_skb(nskb);
		return NULL;
	}

	return nskb;
}
EXPORT_SYMBOL_GPL(wg_decrypt_skb_full);

/* ── XChaCha20-Poly1305 (for cookies) ─────────────────────────────── */

int wg_xchacha20poly1305_encrypt(
	const u8 key[32], const u8 nonce[24], const u8 *src, u32 src_len,
	const u8 *ad, u32 ad_len, u8 *dst)
{
	xchacha20poly1305_encrypt(dst, src, src_len, ad, ad_len, nonce, key);
	return 0;
}
EXPORT_SYMBOL_GPL(wg_xchacha20poly1305_encrypt);

int wg_xchacha20poly1305_decrypt(
	const u8 key[32], const u8 nonce[24], const u8 *src, u32 src_len,
	const u8 *ad, u32 ad_len, u8 *dst)
{
	if (src_len < CHACHA20POLY1305_AUTHTAG_SIZE)
		return -EINVAL;
	if (!xchacha20poly1305_decrypt(dst, src, src_len, ad, ad_len, nonce, key))
		return -EBADMSG;
	return 0;
}
EXPORT_SYMBOL_GPL(wg_xchacha20poly1305_decrypt);

/* ── BLAKE2s ───────────────────────────────────────────────────────── */

void wg_blake2s_hash(const u8 *const *chunks, const u32 *chunk_lens,
	u32 num_chunks, u8 out[32])
{
	struct blake2s_state state;
	u32 i;
	blake2s_init(&state, BLAKE2S_HASH_SIZE);
	for (i = 0; i < num_chunks; i++)
		blake2s_update(&state, chunks[i], chunk_lens[i]);
	blake2s_final(&state, out);
}
EXPORT_SYMBOL_GPL(wg_blake2s_hash);

void wg_blake2s_256_mac(const u8 *key, u32 key_len,
	const u8 *data, u32 data_len, u8 out[32])
{
	blake2s(out, data, key, BLAKE2S_HASH_SIZE, data_len, key_len);
}
EXPORT_SYMBOL_GPL(wg_blake2s_256_mac);

void wg_blake2s_256_hmac(const u8 key[32], const u8 *data, u32 data_len,
	u8 out[32])
{
	struct blake2s_state state;
	u8 padded_key[BLAKE2S_BLOCK_SIZE];
	u8 ipad[BLAKE2S_BLOCK_SIZE];
	u8 opad[BLAKE2S_BLOCK_SIZE];
	u8 inner_hash[BLAKE2S_HASH_SIZE];
	int i;

	memset(padded_key, 0, BLAKE2S_BLOCK_SIZE);
	memcpy(padded_key, key, 32);
	for (i = 0; i < BLAKE2S_BLOCK_SIZE; i++) {
		ipad[i] = padded_key[i] ^ 0x36;
		opad[i] = padded_key[i] ^ 0x5c;
	}
	blake2s_init(&state, BLAKE2S_HASH_SIZE);
	blake2s_update(&state, ipad, BLAKE2S_BLOCK_SIZE);
	blake2s_update(&state, data, data_len);
	blake2s_final(&state, inner_hash);
	blake2s_init(&state, BLAKE2S_HASH_SIZE);
	blake2s_update(&state, opad, BLAKE2S_BLOCK_SIZE);
	blake2s_update(&state, inner_hash, BLAKE2S_HASH_SIZE);
	blake2s_final(&state, out);
	memzero_explicit(padded_key, sizeof(padded_key));
	memzero_explicit(ipad, sizeof(ipad));
	memzero_explicit(opad, sizeof(opad));
	memzero_explicit(inner_hash, sizeof(inner_hash));
}
EXPORT_SYMBOL_GPL(wg_blake2s_256_hmac);

void wg_hkdf(const u8 key[32], const u8 *input, u32 input_len,
	u8 out1[32], u8 out2[32], u8 out3[32])
{
	u8 prk[32], t_input[33];
	wg_blake2s_256_hmac(key, input, input_len, prk);
	t_input[0] = 0x01;
	wg_blake2s_256_hmac(prk, t_input, 1, out1);
	memcpy(t_input, out1, 32);
	t_input[32] = 0x02;
	wg_blake2s_256_hmac(prk, t_input, 33, out2);
	if (out3) {
		memcpy(t_input, out2, 32);
		t_input[32] = 0x03;
		wg_blake2s_256_hmac(prk, t_input, 33, out3);
	}
	memzero_explicit(prk, sizeof(prk));
	memzero_explicit(t_input, sizeof(t_input));
}
EXPORT_SYMBOL_GPL(wg_hkdf);

/* ── Curve25519 ────────────────────────────────────────────────────── */

int wg_curve25519(u8 out[32], const u8 scalar[32], const u8 point[32])
{
	return curve25519(out, scalar, point) ? 0 : -EINVAL;
}
EXPORT_SYMBOL_GPL(wg_curve25519);

void wg_curve25519_generate_secret(u8 secret[32])
{
	curve25519_generate_secret(secret);
}
EXPORT_SYMBOL_GPL(wg_curve25519_generate_secret);

void wg_curve25519_generate_public(u8 pub_key[32], const u8 secret[32])
{
	if (!curve25519_generate_public(pub_key, secret))
		memset(pub_key, 0, 32);
}
EXPORT_SYMBOL_GPL(wg_curve25519_generate_public);

/* ── Random + Time + Secure memory ─────────────────────────────────── */

void wg_get_random_bytes(u8 *buf, u32 len) { get_random_bytes(buf, len); }
EXPORT_SYMBOL_GPL(wg_get_random_bytes);

u64 wg_ktime_get_ns(void) { return ktime_get_ns(); }
EXPORT_SYMBOL_GPL(wg_ktime_get_ns);

void wg_ktime_get_real(s64 *secs, s64 *nsecs)
{
	struct timespec64 ts;
	ktime_get_real_ts64(&ts);
	*secs = ts.tv_sec;
	*nsecs = ts.tv_nsec;
}
EXPORT_SYMBOL_GPL(wg_ktime_get_real);

void wg_memzero(void *ptr, size_t len) { memzero_explicit(ptr, len); }
EXPORT_SYMBOL_GPL(wg_memzero);

int wg_crypto_memneq(const void *a, const void *b, size_t len)
{
	return crypto_memneq(a, b, len);
}
EXPORT_SYMBOL_GPL(wg_crypto_memneq);

int wg_crypto_init(void) { return 0; }
EXPORT_SYMBOL_GPL(wg_crypto_init);

void wg_crypto_exit(void) {}
EXPORT_SYMBOL_GPL(wg_crypto_exit);
