#include <string.h>
#include <stdlib.h>

#include "base.c"

char *str_replace(const char *orig, const char *rep, const char *with)
{
    // source: https://stackoverflow.com/questions/779875/what-function-is-to-replace-a-substring-from-a-string-in-c
    char *result;    // the return string
    const char *ins; // the next insert point
    char *tmp;       // varies
    int len_rep;     // length of rep (the string to remove)
    int len_with;    // length of with (the string to replace rep with)
    int len_front;   // distance between rep and end of last rep
    int count;       // number of replacements

    // sanity checks and initialization
    if (!orig || !rep)
        return NULL;
    len_rep = strlen(rep);
    if (len_rep == 0)
        return NULL; // empty rep causes infinite loop during count
    if (!with)
        with = "";
    len_with = strlen(with);

    // count the number of replacements needed
    ins = orig;
    for (count = 0; (tmp = strstr(ins, rep)); ++count)
    {
        ins = tmp + len_rep;
    }

    tmp = result = malloc(strlen(orig) + (len_with - len_rep) * count + 1);
    // if (!count)
    //     strcpy(result, orig);
    //     return result;

    if (!result)
        return NULL;

    while (count--)
    {
        ins = strstr(orig, rep);
        len_front = ins - orig;
        tmp = strncpy(tmp, orig, len_front) + len_front;
        tmp = strcpy(tmp, with) + len_with;
        orig += len_front + len_rep;
    }
    strcpy(tmp, orig);
    return result;
}

struct ReplaceAction
{
    struct result (*eval)(const struct ReplaceAction *, const signed long *);
    char trg[16];
    char with[16];
};
struct result replace_action_eval(const struct ReplaceAction *action, const signed long *start)
{
    char orig[16];
    sprintf(orig, "%d", *start);

    const char *trg_pointer = (*action).trg;
    const char *with_pointer = (*action).with;

    char *replaced = str_replace(orig, trg_pointer, with_pointer);

    struct result out = {atol(replaced), 1};
    return out;
};
void replace_action_print(const struct ReplaceAction *action){
    printf("Replace %s -> %s\n", action->trg, action->with);
};
