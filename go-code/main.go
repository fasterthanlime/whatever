package main

//#cgo LDFLAGS: ${SRCDIR}/../target/debug/libwhatever.a -lpthread -ldl
//#include <stdlib.h>
//#include "main.h"
import "C"

import (
	"log"
	"reflect"
	"unsafe"
)

func main() {
	log.Printf("Hello from Go")

	s := "The moonlight 🦜"

	b := []byte(s)
	C.do_whatever((*C.char)(unsafe.Pointer(&b[0])), (C.size_t)(len(b)))

	// cs := C.CString(s)
	// defer C.free(unsafe.Pointer(cs))
	// C.do_whatever(cs, (C.size_t)(len(s)))

	rs := C.returns_whatever()

	sh := reflect.StringHeader{
		Data: uintptr(unsafe.Pointer(C.rstring_data(rs))),
		Len:  int(C.rstring_len(rs)),
	}
	s2 := *(*string)(unsafe.Pointer(&sh))

	log.Printf("Rust gave us this string: %v", s2)
	C.rstring_free(rs)
	log.Printf("Rust took back this string: %v", s2)
}
