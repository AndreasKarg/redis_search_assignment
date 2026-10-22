#include <stdint.h>
#include <stdio.h>
#include <inttypes.h>

#include "trie_rs_header_wrapper.h"

int main(void)
{
    uint64_t sum = add(35, 7);

    printf("The sum is: %" PRId64 "\n", sum);
    return 0;
}
