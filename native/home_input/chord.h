#ifndef NEARBY_HOME_CHORD_H
#define NEARBY_HOME_CHORD_H
#include <stdbool.h>

enum { NEARBY_X = 2, NEARBY_FN = 16 };
struct nearby_chord { bool function; bool consumed_x; };

/* Consume the complete X gesture, including release after FN is released. */
static bool nearby_consume(struct nearby_chord *state, unsigned button, bool down) {
    if (button == NEARBY_FN) {
        state->function = down;
        return false;
    }
    if (button != NEARBY_X) return false;
    if (down && state->function) state->consumed_x = true;
    bool consume = state->consumed_x;
    if (!down) state->consumed_x = false;
    return consume;
}
#endif
