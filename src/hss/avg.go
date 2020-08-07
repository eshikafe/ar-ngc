// Copyright (c) 2020 ngc project
// AVG - HSS Authentication Vector Generator

package hss

import "ngc/lib/milenage"

// Quintet = (RAND, XRES, CK, IK, AUTN)
type Quintet struct {
	rand []byte
	xres []byte // from milenage f2
	ck   []byte // from milenage f3
	ik   []byte // from milenage f4
	autn []byte // SQN ^ AK || AMF|| MAC-A
}

// The A4 algorithm used to decrypt the encrypted user secret key is AES128
// Sensitive user data is stored encrypted in the database, so it needs to be decrypted to be used in the generation of quintets

func GenerateQuintet(ak, amf []byte) Quintet {
	var q Quintet

	macA := milenage.F1()
	xRes := milenage.F2()

}
