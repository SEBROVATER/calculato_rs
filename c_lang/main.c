#include <stdio.h>


#include "types.c"



int main(void)
{

    const static signed long start = -1235;
    const static signed long finish = -18;
    const char count = 5;
    const char possible_count = 6;
    MixedAction actions[possible_count];

    actions[0].type = AddValueActionType;

    struct AddValueAction action1 = {add_value_action_eval, -3};
    actions[1].data.add_value_action = action1;


    struct BackspaceAction action2 = {backspace_action_eval};
    actions[2].data.backspace_action = action2;
    struct ReplaceAction action3 = {replace_action_eval, "23", "1"};
    actions[3].data.replace_action = action3;
    struct InsertAction action4 = {insert_action_eval, "34"};
    actions[4].data.insert_action = action4;
    struct SignInvAction action5 = {signinv_action_eval};
    actions[5].data.sign_inv_action = action5;
    struct SumAction action6 = {sum_action_eval};
    actions[6].data.sum_action = action6;

    // struct  AllActions * ref = &actions[2];

    {
        struct result res = action1.eval(&action1, &start);
        if (res.status)
        {
            printf("%d\n", res.value);
        };
    }

    {
        struct result res = action2.eval(&action2, &start);
        if (res.status)
        {
            printf("%d\n", res.value);
        };
    }

    {
        struct result res = action3.eval(&action3, &start);
        if (res.status)
        {
            printf("%d\n", res.value);
        };
    }
    {
        struct result res = action4.eval(&action4, &start);
        if (res.status)
        {
            printf("%d\n", res.value);
        };
    }
    {
        struct result res = action5.eval(&action5, &start);
        if (res.status)
        {
            printf("%d\n", res.value);
        };
    }
    {
        struct result res = action6.eval(&action6, &start);
        if (res.status)
        {
            printf("%d\n", res.value);
        };
    }

    return 0;
}