// Copyright (c) 2020 ngc project

package hss

import "log"


var initialized bool = false

//log.SetFlags(Ldate | Ltime | Lmicroseconds | Llongfile)

// Start HSS server
func Start() bool {
	rv, err := hssContextInit()
	if err != nil {
		log.Fatal(err)
		return rv
	}

	rv, err := hssContextParseConfig()
	if err != nil {
		log.Fatal(err)
		return rv
	}

	rv, err := hssDbInit()
	if err != nil {
		log.Fatal(err)
		return rv
	}

	rv := hssFdInit()
	if err != nil {
		log.Fatal(err)
		return rv
	}
	initialized = true
	return initialized

}

// Stop HSS server
func Stop() {
	if initialized != true {
		log.
		return
	}
}
