// https://github.com/acetcom/open5gs/blob/master/lib/crypt/milenage.c

package security

import (
	"ngc/lib/security/aes"
)

// int aes_128_encrypt_block(const uint8_t *key,
//    const uint8_t *in, uint8_t *out)

/*
	{
    const int key_bits = 128;
    unsigned int rk[OGS_AES_RKLENGTH(128)];
    int nrounds;

    nrounds = ogs_aes_setup_enc(rk, key, key_bits);
    ogs_aes_encrypt(rk, nrounds, in, out);

    return 0;
}
*/

func aes128EncryptBlock(key []byte, in []byte, out []byte) int {
	const key_bits int = 128

	var rk uint32 = make([]uint32, aes.RkLength(128))
	var nrounds = aes.
}
