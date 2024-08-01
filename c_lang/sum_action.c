#include <stdlib.h>

#include "base.c"

struct SumAction
{
    struct result (*eval)(const struct SumAction *, const signed long *);
};
struct result sum_action_eval(const struct SumAction *action, const signed long *start)
{
    signed long quot;
    signed long rem = 0;
    quot = *start;
    signed long accum = 0;
    div_t res;
    while (quot != 0)
    {
        res = div(quot, 10);
        quot = res.quot;
        accum += res.rem;
    }

    struct result out = {accum, 1};
    return out;
};
void sum_action_print(const struct SumAction *action){
    printf("Sum\n");
};
