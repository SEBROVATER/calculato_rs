#ifndef BASE_INCLUDED
#define BASE_INCLUDED

struct result
{
    const signed long value;
    const char status;
};

typedef enum {
    AddValueActionType,
    BackspaceActionType,
    InsertActionType,
    ReplaceActionType,
    SignInvActionType,
    SumActionType,
    
} ActionType;


#endif