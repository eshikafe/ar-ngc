// Copyright (c) 2020 ngc project

package hss

import (
	"errors"
	"log"
)


var initialized bool = false
var contextInitialized bool = false


//log.SetFlags(Ldate | Ltime | Lmicroseconds | Llongfile)

// Start HSS server
func Start() bool {
	rv, err := hssContextInit()
	if err != nil {
		log.Fatal(err)
		return rv
	}

	// rv, err := hssContextParseConfig()
	// if err != nil {
	// 	log.Fatal(err)
	// 	return rv
	// }

	// rv, err := hssDbInit()
	// if err != nil {
	// 	log.Fatal(err)
	// 	return rv
	// }

	// rv := hssFdInit()
	// if err != nil {
	// 	log.Fatal(err)
	// 	return rv
	// }
	// initialized = true
	// return initialized

}

// Stop HSS server
func Stop() {
	if initialized != true {
		log.
		return
	}
}



func hssContextInit() (bool, error) {
	if contextInitialized != true {
		contextInitialized = true
		return contextInitialized, nil
	} else {
		return false, errors.New("HSS context already active")
	}
}

// func hssContextParseConfig() {

// }

// func hssDbInit() {

// }

// func hssFdInit() {

// }