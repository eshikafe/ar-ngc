// Copyright (c) 2020 ngc project
// Reference:
// https://github.com/acetcom/open5gs/blob/master/lib/crypt/milenage.c
// Milenage algorithm (3GPP TS 35.205, .206, .207, .208)

package milenage

import (
	"ngc/lib/security/aes" // use crypt/aes
)

func aes128EncryptBlock(key []byte, in []byte, out []byte) int {
	const keyBits int = 128
	rk := make([]uint32, aes.RkLength(128))
	var nrounds = aes.SetupEnc(rk, key, keyBits)
	aes.Encrypt(rk, nrounds, in, out)
	return 0
}

// Description: Milenage security function F1.
//   Computes network authentication code MAC-A from key K,
//   random challenge RAND, sequence number SQN, and
//  authentication management field AMF.
// Document Reference: 35.206 v10.0.0 Annex 3
func milenageF1(opc, k, _rand, sqn, amf, mac_a, mac_s []uint8) int {
	return 0
}
