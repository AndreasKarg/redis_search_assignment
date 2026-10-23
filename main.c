#include <stdint.h>
#include <stdio.h>
#include <inttypes.h>

#include "trie_rs.h"

int main(void)
{
    TrieNode *node = TrieNode_New();

    uint64_t sum = add(35, 7);

    const size_t payload_len = 128;
    char *payload = malloc(payload_len);
    snprintf(payload, payload_len, "The sum is: %" PRId64 "\n", sum);

    size_t runes_length;
    Rune *runes = strToRunes(payload, &runes_length);

    // Note to reviewer: Casting `char*` to RSPayload just for the demo case
    TrieNode_Add(&node, runes, runes_length, (RSPayload*)payload, 123.4f, ADD_INCR, free);

    // Now get the payload back
    char *retrieved_payload = (char*)TrieNode_GetValue(node, runes, runes_length, false);

    printf("%s\n", retrieved_payload);

    runeFree(runes);
    TrieNode_Free(node, free);

    return 0;
}
