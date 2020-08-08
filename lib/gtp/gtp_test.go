// Copyright (c) 2020 ngc project
package gtp

import (
	"fmt"
	"testing"
)

func TestGTPVersion(t *testing.T) {
	v := GTPVersion()
	fmt.Println(v)
}
