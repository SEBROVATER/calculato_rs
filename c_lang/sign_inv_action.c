#include "base.c"

struct SignInvAction
{
    struct result (*eval)(const struct SignInvAction *, const signed long *);
};
struct result signinv_action_eval(const struct SignInvAction *action, const signed long *start)
{

    struct result out = {(*start) * -1, 1};
    return out;
};
void signinv_action_print(const struct SignInvAction *action){
    printf("Sign invert\n");
};