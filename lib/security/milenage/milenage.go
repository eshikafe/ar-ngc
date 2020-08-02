// Copyright (c) 2020 ngc project
// Reference:
// 3GPP TS 35.205 - Document 1: General
// 3GPP TS 35.206 - Document 2: Algorithm Specification
// 3GPP TS 35.207 - Document 3: Implementors' Test Data
// 3GPP TS 35.208 - Document 4: Design Conformance Test Data

package milenage

import (
	"crypto/aes"
)

// Rijndael Encryption algorithm => AES (Advanced Encryption Standard)
// 3GPP TS 35.206 4.2
// Apply the Rijndael block cipher encryption to the input value x using the key k.
// Returns a 128-bit output 
func aesEncrypt(x [16]uint8, k [16]uint8) [16]uint8 {
	// 128-bit output
	var out[16]uint8 
	// create 128-bit cipher block => AES-128
	c, err = aes.NewCipher(k)
	// encrypt x using c
	c.Encrypt(out, x)
	return out
}

// TS 35.206 2.2.4
// Cyclic rotation of 128-bit value x by r bit positions (in bytes) 
// towards the most significant bit. 
// If x = x[0] || x[1] || … x[127], and y = rot(x,r),
// then y = x[r] || x[r+1] || … x[127] || x[0] || x[1] || x[r-1]. 
func rot(x [16]uint8, r uint8) []uint8 {
	y := append(x[r:16],x[0:r]...)
	return y
}


// Milenage security (authentication and key generation) functions
// 3GPP TS 35.205 3
// Parameters:
//  RAND: Random Challenge (128 bits)
//  AMF: Authentication Management Field (16 bits)
//  SQN: Sequence Number (48 bits)
//  K: Subscriber key (128 bits)
//  OP:  Operator Variant Algorithm Configuration Field (128 bits)
//  OPc:  Derived from OP and K (128 bits)
//  MAC-A: Network Authentication Code (64 bits)
//  MAC-S: Resynchronisation Authentication Code (64 bits / 8 bytes)

func milenageF1(opc [16]uint8, k [16]uint8, rand [16]uint8, sqn [6]uint8, amf [2]uint8) [16]uint8 {
	var out1, c1 [16]uint8
	var mac_a [8]uint8
	var in1, temp [16]uint8
	var r1 byte = 8 // 64 bits

	// in1 = SQN||AMF||SQN||AMF
    copy(in1[0:], sqn[0:6]) // in1[0]..in1[5] = sqn[0]..sqn[5]
	copy(in1[6:], amf[0:2]) // in1[6]..in1[7] = amf[0]..amf[1]
	copy(in1[8:], in1[0:8]) // in1[8]..in1[13] = sqn[0]..sqn[5],in1[14]..in[15] = amf[0]..amf[1]

	// temp = E[rand XOR OPc]k = rijndaelEncrypt((rand xor opc), k)
	for i := 0 i < 16; i++ {
		temp[i] = rand[i] ^ opc[i]
	}
	temp = aesEncrypt(temp, k)
	
	//OUT1 = E[temp xor rot(in1 xor opc, r1) xor c1]k xor opc
	for i := 0; i < 16; i++ {
		in1[i] = in1[i] ^ opc[i]
	}
	in1 = rot(in1, r1)
	for i := 0; i < 16; i++ {
		temp[i] = temp[i] ^ in1[i] ^ c1[i]
	}
	temp = aesEncrypt(temp, k)

	for i := 0; i < 16; i++ {
		out1[i] = temp[i] ^ opc[i]
	}
	
}

// Name: f1 (the network authentication function)
// Function: MAC-A = f1(OPC, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 4.1
// OPc: computed off USIM (TS 35.206 5.1)
func f1(opc [16]uint8, k [16]uint8, rand [16]uint8, sqn [6]uint8, amf [2]uint8) [8]uint8 {
	var mac_a [8]uint8
	out1 := milenageF1(opc, k, rand, sqn, amf)
	copy(mac_a[0:8], out1[0:8])
	return mac_a

}

// Name: f1* (the re-synchronisation message authentication function;)
// Function: MAC-S = f1*(OPc, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 v10.0.0 Annex 3
func f1Star(opc [16]uint8, k [16]uint8, rand [16]uint8, sqn_ms [6]uint8, amf_star [2]uint8) [8]uint8 {
	var mac_s [8]uint8
	out1 := milenageF1(opc, k, rand, sqn_ms, amf_star)
	copy(mac_s[0:8], out1[8:])
	return mac_s
	return 0
}
