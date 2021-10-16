// Copyright (c) 2021 Austin Aigbe
// Reference:
// 3GPP TS 35.205 - Document 1: General
// 3GPP TS 35.206 - Document 2: Algorithm Specification
// 3GPP TS 35.207 - Document 3: Implementors' Test Data
// 3GPP TS 35.208 - Document 4: Design Conformance Test Data

package milenage

import (
	"crypto/aes"
	"fmt"
	"log"
)

// GenerateRAND return a 128-bit pseudo random number. It implements f0
// TODO: Not yet implemented
func GenerateRAND() []byte {
	return make([]byte, 16)
}

// GenenrateSQN generate the SQN (48-bit) used as input for quintet generation obtained as follows:
// SQN = SEQ (Global part) || IND (Individual part)
// TODO: Not yet implemented
func GenenrateSQN() []byte {
	return make([]byte, 8)
}

// AESEncrypt implements the Rijndael Encryption algorithm (3GPP TS 35.206 4.2)
// Apply the Rijndael block cipher encryption to the input value x using the key k.
// Returns a 128-bit output
func AESEncrypt(x, k []byte) []byte {
	// 128-bit output
	var out = make([]byte, 16)
	// create 128-bit cipher block => AES-128
	c, err := aes.NewCipher(k)
	if err != nil {
		log.Fatal(err)
	}
	// encrypt x using c
	c.Encrypt(out, x)
	return out
}

// TS 35.206 2.2.4
// Cyclic rotation of 128-bit value x by r bit positions (in bytes)
// towards the most significant bit.
// If x = x[0] || x[1] || … x[127], and y = rot(x,r),
// then y = x[r] || x[r+1] || … x[127] || x[0] || x[1] || x[r-1].
func rot(x []byte, r byte) []byte {
	y := append(x[r:16], x[0:r]...)
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

//  TS 35.206 4.1 Milenage Algorithm
func milenageFn(fn string, opc, k, rand, sqn, amf []byte) []byte {
	// 128-bit variables
	var in1 []byte = make([]byte, 16)
	var temp = make([]byte, 16)
	var out1 = /*, out2, out3, out4, out5*/ make([]byte, 16)
	var c1 []byte = make([]byte, 16)
	var c2 []byte = make([]byte, 16)
	var c3 []byte = make([]byte, 16)
	var c4 []byte = make([]byte, 16)
	var c5 []byte = make([]byte, 16)

	// Five integers r1, r2, r3, r4, r5 are defined as follows:
	// var r1, r2, r3, r4, r5 byte = 8, 0, 4, 8, 12
	var r1 byte = 8

	// An intermediate 128-bit value TEMP is computed as follows:
	// temp = E[rand XOR OPc]k = rijndaelEncrypt((rand xor opc), k)
	for i := 0; i < 16; i++ {
		temp[i] = rand[i] ^ opc[i]
	}
	temp = AESEncrypt(temp, k)
	fmt.Printf("1st encryption (actual)    : %x\n", temp)
	// A 128-bit value IN1 is constructed as follows:
	// in1 = SQN||AMF||SQN||AMF
	copy(in1[0:], sqn[0:6]) // in1[0]..in1[5] = sqn[0]..sqn[5]
	copy(in1[6:], amf[0:2]) // in1[6]..in1[7] = amf[0]..amf[1]
	copy(in1[8:], in1[0:8]) // in1[8]..in1[13] = sqn[0]..sqn[5],in1[14]..in[15] = amf[0]..amf[1]

	// Five 128-bit constants c1, c2, c3, c4, c5 are defined as follows:
	// c1[i] = 0 for i = 0..127

	// c2[i] = 0 for i = 0..126, c2[127] = 1
	c2[15] = 1 // 0b00000001

	// c3[i] = 0 for 0 ≤ i ≤ 127, except that c3[126] = 1
	c3[15] = 2 // 0b00000010

	// c4[i] = 0 for 0 ≤ i ≤ 127, except that c4[125] = 1
	c4[15] = 4 // 0b00000100

	// c5[i] = 0 for 0 ≤ i ≤ 127, except that c5[124] = 1
	c5[15] = 8 // 0b00001000

	// OUT1 = E[TEMP ⊕ rot(IN1 ⊕ OPC, r1) ⊕ c1]K ⊕ OPC
	for i := 0; i < 16; i++ {
		in1[i] = in1[i] ^ opc[i]
	}
	in1 = rot(in1, r1)
	for i := 0; i < 16; i++ {
		temp[i] = temp[i] ^ in1[i] ^ c1[i]
	}
	temp = AESEncrypt(temp, k)
	for i := 0; i < 16; i++ {
		out1[i] = temp[i] ^ opc[i]
	}
	return out1

	// OUT2 = E[rot(TEMP⊕ OPC, r2) ⊕ c2]K ⊕ OPC
}

// F0 the random challenge generating function
// Returns RAND: Random Challenge (128 bits)
func F0() []byte {
	return GenerateRAND()
}

// F1 (the network authentication function)
// 64-bit MAC-A = f1(OPC, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 4.1
// OPc: computed off USIM (TS 35.206 5.1)
func F1(opc, k, rand, sqn, amf []byte) []byte {
	var macA = make([]byte, 8)
	out1 := milenageFn("f1", opc, k, rand, sqn, amf)
	copy(macA[0:8], out1[0:8])
	return macA

}

// F1Star (the re-synchronisation message authentication function;)
// 64-bit MAC-S = f1*(OPc, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 v10.0.0 Annex 3
func F1Star(opc, k, rand, sqnMs, amfStar []byte) []byte {
	var macS = make([]byte, 8)
	out1 := milenageFn("f1*", opc, k, rand, sqnMs, amfStar)
	copy(macS[0:8], out1[8:])
	return macS
}

// F2 (the user authentication function)
// 64-bit RES = f2(OPc, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 4.1
func F2(opc, k, rand, sqnMs, amfStar []byte) []byte {
	var res = make([]byte, 8)
	out2 := milenageFn("f2", opc, k, rand, sqnMs, amfStar)
	copy(res[0:8], out2[8:])
	return res
}

// F3 (the cipher key derivation function)
// 128-bit CK= f3(OPc, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 4.1
func F3(opc, k, rand, sqnMs, amfStar []byte) []byte {
	var ck = make([]byte, 16)
	out3 := milenageFn("f3", opc, k, rand, sqnMs, amfStar)
	copy(ck[0:16], out3[0:16])
	return ck
}

// F4 (the integrity key derivation function)
// 128-bit IK = f4(OPc, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 v10.0.0 Annex 3
func F4(opc, k, rand, sqnMs, amfStar []byte) []byte {
	var ik = make([]byte, 16)
	out4 := milenageFn("f4", opc, k, rand, sqnMs, amfStar)
	copy(ik[0:16], out4[0:16])
	return ik
}

// F5 (the anonymity key derivation function)
// 48-bit AK = f5(OPc, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 4.1
func F5(opc, k, rand, sqnMs, amfStar []byte) []byte {
	var ak = make([]byte, 6)
	out2 := milenageFn("f2", opc, k, rand, sqnMs, amfStar)
	copy(ak[0:6], out2[0:6])
	return ak
}

// F5Star (the anonymity key derivation function for the re-synchronisation message)
// 48-bit AK = f5*(OPc, RAND, AMF, SQN, K)
// 3GPP Reference: 35.206 4.1
func F5Star(opc, k, rand, sqnMs, amfStar []byte) []byte {
	var ak = make([]byte, 6)
	out5 := milenageFn("f5*", opc, k, rand, sqnMs, amfStar)
	copy(ak[0:6], out5[0:6])
	return ak
}
