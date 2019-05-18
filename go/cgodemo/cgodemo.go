package cgodemo

/*
#include "cpp/basic_c.h"
*/
import "C"
import "unsafe"

func GetCMessage() string {
	var msg *C.char = C.get_message()
	defer C.free(unsafe.Pointer(msg))
	return C.GoString(msg)
}