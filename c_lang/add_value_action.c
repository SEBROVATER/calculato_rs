#include "base.c"

struct AddValueAction
{
    struct result (*eval)(const struct AddValueAction *, const signed long *);
    signed long value;
};
struct result add_value_action_eval(const struct AddValueAction *action, const signed long *start)
{
    struct result out = {*start + (*action).value, 1};
    return out;
};

void add_value_action_print(const struct AddValueAction *action){
    printf("Add Value %ld\n", action->value);
};
