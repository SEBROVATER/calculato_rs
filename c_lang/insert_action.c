#include <string.h>
#include <stdlib.h>

#include "base.c"

struct InsertAction
{
    struct result (*eval)(const struct InsertAction *, const signed long *);
    char value[16];
};
struct result insert_action_eval(const struct InsertAction *action, const signed long *start)
{
    char string_start[16];
    sprintf(string_start, "%d", *start);

    strcat(string_start, (*action).value);

    struct result out = {atol(string_start), 1};
    return out;
};
void insert_action_print(const struct InsertAction *action){
    printf("Insert %s\n", action->value);
};