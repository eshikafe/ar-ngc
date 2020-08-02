// ngc
// hss.go

package hss

import (
	"crypto/hmac"
	"crypto/sha256"
)

const FC_VALUE = 0x10

func HssAuthVec(ck []byte, ik []byte, plmn_id [3]byte, sqn []byte, ak []byte, kasme []byte) {
	s := make([]byte, 14)
	k := make([]byte, 32)

	var i int

	s[0] = FC_VALUE

	s[4] = 0x00
	s[5] = 0x03

	for i := 0; i < 6; i++ {
		s[6+i] = sqn[i] ^ ak[i]
	}
	s[12] = 0x00
	s[13] = 0x06
	mac := hmac.New(sha256.New, k)
}
