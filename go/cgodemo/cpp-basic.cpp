#include "cpp-basic.h"
#include "cpp/basic_library.h"

char* get_message() {
    InfoPrinter printer;
    std::string s = printer.getString();
    char* cstr = new char[s.length()+1];
    std::strcpy(cstr, s.c_str());
    return cstr;
}