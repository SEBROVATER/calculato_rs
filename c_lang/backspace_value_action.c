#include "base.c"

struct BackspaceAction
{
    struct result (*eval)(const struct BackspaceAction *, const signed long *);
};
struct result backspace_action_eval(const struct BackspaceAction *action, const signed long *start)
{

    struct result out = {*start / 10, 1};
    return out;
};
void backspace_action_print(const struct BackspaceAction *action){
    printf("Backspace\n");
};