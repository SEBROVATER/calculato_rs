#include <stdio.h>

#include "types.c"





int check_combination(int *indexes, int r, MixedAction *actions, const signed long start, const signed long finish) {
    signed long tmp = start;
    for (int j = 0; j < r; j++) {
        struct result res = eval_mixed_action(&actions[j], tmp);

        if (!res.status) {
            return 0;
        };
        tmp = res.value;

    }
    if (tmp != finish){
        return 0;
    }
    for (int j = 0; j<r; j++) {
        printf("--- SOLUTION END ---\n");
        print_mixed_action(&actions[j]);
        printf("--- SOLUTION END ---\n");
    }
    return 1;
}

void generateCombinations(int n, int r, int index, int *data, int i, MixedAction *actions, const signed long start, const signed long finish) {
    // Base case: when combination of size r is formed
    if (index == r) {
        check_combination(data, r, actions, start, finish);
        return;
    }

    // When no more elements are there to put in data
    if (i >= n)
        return;

    // Current is included, put next at next location
    data[index] = i;
    generateCombinations(n, r, index + 1, data, i, actions, start, finish);

    // Current is excluded, replace it with the next
    // (Note that i+1 is passed, but index is not changed)
    generateCombinations(n, r, index, data, i + 1, actions, start, finish);
}

// Function to print all combinations of size r in arr[] of size n
void combinationUtil(int n, int r, MixedAction *actions, const signed long start, const signed long finish) {
    // Temporary array to store combinations
    int data[r];

    // Print all combinations using temporary array 'data'
    generateCombinations(n, r, 0, data, 0,actions, start, finish);
}

