package milenage

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"log"
	"testing"
)

func strToHexArray(h string) []byte {
	n, err := hex.DecodeString(h)
	if err != nil {
		log.Fatal(err)
	}
	return n
}

// Rijndael test
func TestRijndaelTestSet1(t *testing.T) {
	// Key: 465b5ce8 b199b49f aa5f0a2e e238a6bc
	// Plaintext: ee36f7cf 037d37d3 692f7f03 99e7949a

	key := strToHexArray("465b5ce8b199b49faa5f0a2ee238a6bc")
	plaintext := strToHexArray("ee36f7cf037d37d3692f7f0399e7949a")
	ciphertext := strToHexArray("9e2980c59739da67b136355e3cede6a2")
	res := AESEncrypt(plaintext, key)

	if !bytes.Equal(res, ciphertext) {
		t.Error("3GPP TS 35.207 Rijndael Test Set 1 failed\n\n")
	} else {
		fmt.Print("3GPP TS 35.207 Rijndael Test Set 1 Passed\n\n")
	}

}
func TestF1TestSet1(t *testing.T) {
	// 3GPP TS 35.207 4.3 Test Set 1
	// K: 465b5ce8 b199b49f aa5f0a2e e238a6bc
	// RAND: 23553cbe 9637a89d 218ae64d ae47bf35
	// SQN: ff9bb4d0 b607
	// AMF: b9b9
	// OP: cdc202d5 123e20f6 2b6d676a c72cb318
	// SQN,AMF expanded to 128 bits ff9bb4d0 b607b9b9 ff9bb4d0 b607b9b9
	// OPC cd63cb71 954a9f4e 48a5994e 37a02baf

	k := strToHexArray("465b5ce8b199b49faa5f0a2ee238a6bc")
	rand := strToHexArray("23553cbe9637a89d218ae64dae47bf35")
	amf := strToHexArray("b9b9")
	opc := strToHexArray("cdc202d5123e20f62b6d676ac72cb318")
	sqn := strToHexArray("ff9bb4d0b607")

	// Value after 1st
	// encryption 9e2980c5 9739da67 b136355e 3cede6a2
	// (SQN,AMF) XOR OPC, rotated b73e2d9e 81a79216 32f87fa1 234d26f7
	// Input to 2nd encryption 2917ad5b 169e4871 83ce4aff 1fa0c055
	// Output of 2nd encryption 87fc31b2 c19530fd 496a36d0 f3485a46
	// Value of f1 4a9ffac3 54dfafb3
	// Value of f1* 01cfaf9e c4e871e9

	macAE := strToHexArray("4a9ffac354dfafb3")
	//macSE := strToHexArray("01cfaf9ec4e871e9")
	macAA := F1(opc, k, rand, sqn, amf)
	//macSA := F1Star(opc, k, rand, sqn, amf)

	// fmt.Printf("Mine f1(Actual)  : %x\n", macAA)
	// fmt.Printf("f1 (Expected): %x\n", macAE)

	// fmt.Printf("f1* (Actual)  : %x\n", macSA)
	// fmt.Printf("f1* (Expected): %x\n", macSE)
	if !bytes.Equal(macAE, macAA) {
		t.Error("3GPP TS 35.207 Test Set 1 failed\n")
	} else {
		fmt.Print("3GPP TS 35.207 Test Set 1 Passed\n")
	}
}
