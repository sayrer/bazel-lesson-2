#pragma once

#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif
    char* get_message();
    void free_message(char*);
#ifdef __cplusplus
}
#endif