#pragma once
#include <stddef.h>

/* A payload object is set either by a query expander or by the user, and can be used to process
 * scores. For examples, it can be a feature vector that is then compared to a feature vector
 * extracted from each result or document */
typedef struct {
    char *data;
    size_t len;
} RSPayload;
