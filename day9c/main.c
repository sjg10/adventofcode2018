#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>


typedef struct sMarble {
    uint32_t val;
    struct sMarble * acw;
    struct sMarble * cw;
} Marble;



void compute_game(uint32_t players, uint32_t marbles, uint32_t * scores) {
    memset(scores, 0, players * sizeof(scores[0]));
    /* Initialise the zero marble */
    Marble * current_marble = (Marble *) malloc(sizeof(Marble));
    current_marble->val = 0;
    current_marble->cw = current_marble;
    current_marble->acw = current_marble;
    uint32_t player = 0;
    for (uint32_t i = 1; i <= marbles; i++) {
        if (i % 23 != 0) {
            Marble * new_marble = (Marble *) malloc(sizeof(Marble));
            Marble * left_marble = current_marble->cw;
            Marble * right_marble = current_marble->cw->cw;
            left_marble->cw = new_marble;
            right_marble->acw = new_marble;
            new_marble->cw = right_marble;
            new_marble->acw = left_marble;
            new_marble->val = i;
            current_marble = new_marble;
        }
        else {
            scores[player] += i;
            current_marble  = current_marble->acw->acw->acw->acw->acw->acw->acw;
            scores[player] += current_marble->val;
            current_marble->acw->cw = current_marble->cw;
            current_marble->cw->acw = current_marble->acw;
            Marble * new_marble = current_marble->cw;
            free(current_marble);
            current_marble = new_marble;
        }
        /* Move to the next player */
        player++; player %= players;
    }
    /* Free all the marbles */
    Marble * deleted_marble = current_marble;
    do {
        Marble * next_marble = deleted_marble->cw;
        free(deleted_marble);
        deleted_marble = next_marble;
    }
    while (deleted_marble != current_marble);
}

int main(int argc, char ** argv) {
    int players = atoi(argv[1]);
    int marbles = atoi(argv[2]);
    printf("%d players, %d marbles\n", players, marbles);
    uint32_t * scores = (uint32_t *) malloc(players * sizeof(scores[0]));
    compute_game(players, marbles, scores);
    /* get the max score */
    uint32_t max_score = 0;
    for(uint32_t i = 0; i< players; i++) {
        if (scores[i] > max_score) {
            max_score = scores[i];
        }
    }
    printf("Max score %u\n", max_score);
    return 0;
}

