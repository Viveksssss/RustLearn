#include <stdio.h>
#include <zlib.h>

#ifdef __cplusplus
extern "C" {
#endif

int my_compress(unsigned char *dest, unsigned long *destLen, unsigned char const *source,
    unsigned long sourceLen) {
    return compress(dest, destLen, source, sourceLen);
}

#ifdef __cplusplus
}
#endif
