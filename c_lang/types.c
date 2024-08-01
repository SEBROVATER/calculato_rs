
#include "add_value_action.c"
#include "backspace_value_action.c"
#include "insert_action.c"
#include "replace_action.c"
#include "sign_inv_action.c"
#include "sum_action.c"

#include "base.c"

#ifndef TYPES_INCLUDED
#define TYPES_INCLUDED

typedef union
{
    struct AddValueAction add_value_action;
    struct BackspaceAction backspace_action;
    struct InsertAction insert_action;
    struct ReplaceAction replace_action;
    struct SignInvAction sign_inv_action;
    struct SumAction sum_action;
} AllActions;

typedef struct
{
    ActionType type;
    AllActions data;
} MixedAction;

struct result eval_mixed_action(MixedAction *action, const signed long start)
{
    switch (action->type)
    {
    case AddValueActionType:
    {
        struct result res = action->data.add_value_action.eval(&action->data.add_value_action, &start);
        return res;
    }

    case BackspaceActionType:
    {
        struct result res = action->data.backspace_action.eval(&action->data.backspace_action, &start);
        return res;
    }

    case InsertActionType:
    {
        struct result res = action->data.insert_action.eval(&action->data.insert_action, &start);
        return res;
    }

    case ReplaceActionType:
    {
        struct result res = action->data.replace_action.eval(&action->data.replace_action, &start);
        return res;
    }

    case SignInvActionType:
    {
        struct result res = action->data.sign_inv_action.eval(&action->data.sign_inv_action, &start);
        return res;
    }

    case SumActionType:
    {
        struct result res = action->data.sum_action.eval(&action->data.sum_action, &start);
        return res;
    }
    default:
    {
        struct result res = {start, 0};
        return res;
    }
    }
}

void print_mixed_action(MixedAction *action)
{
    switch (action->type)
    {
    case AddValueActionType:
    {
        add_value_action_print(&action->data.add_value_action);
        break;
    }

    case BackspaceActionType:
    {
        backspace_action_print(&action->data.backspace_action);
        break;
    }
    case InsertActionType:
    {
        insert_action_print(&action->data.insert_action);
        break;
    }
    case ReplaceActionType:
    {
        replace_action_print(&action->data.replace_action);
        break;
    }
    case SignInvActionType:
    {
        signinv_action_print(&action->data.sign_inv_action);
        break;
    }
    case SumActionType:
    {
        sum_action_print(&action->data.sum_action);
        break;
    }

    }
}

#endif