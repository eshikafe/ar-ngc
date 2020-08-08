// Copyright (c) 2020 ngc project
// AVG - HSS Authentication Vector Generator

package hss

import "ngc/lib/milenage"

// Quintet = (RAND, XRES, CK, IK, AUTN)
type Quintet struct {
	rand []byte // (128-bit)
	xres []byte // from milenage f2 (64-bit)
	ck   []byte // from milenage f3 (128-bit)
	ik   []byte // from milenage f4 (128-bit)
	autn []byte // SQN ^ AK || AMF|| MAC-A (128-bit = 48||16||64)
}

// The A4 algorithm used to decrypt the encrypted user secret key is AES128
// Sensitive user data is stored encrypted in the database, so it needs to be decrypted to be used in the generation of quintets

func GenerateQuintet(opc, k, rand, sqn, amf []byte) Quintet {
	var quintet Quintet
	tmp := make([]byte, 6)
	autn := make([]byte, 16)

	ak := milenage.F5(opc, k, rand, sqn, amf)
	mac := milenage.F1(opc, k, rand, sqn, amf)

	// SQN ^ AK || AMF|| MAC-A
	for i := 0; i < 6; i++ {
		tmp[i] = sqn[i] ^ ak[i]
	}
	copy(autn[0:6], tmp[0:6])
	copy(autn[6:8], amf[0:2])
	copy(autn[8:16], mac[0:8])

	quintet.rand = rand
	quintet.xres = milenage.F2(opc, k, rand, sqn, amf)
	quintet.ck = milenage.F3(opc, k, rand, sqn, amf)
	quintet.ik = milenage.F4(opc, k, rand, sqn, amf)
	quintet.autn = autn

	return quintet
}
