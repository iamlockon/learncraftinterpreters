#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

typedef struct doubly_linked_list {
    struct doubly_linked_list* next;
    struct doubly_linked_list* prev;
    const char* value;
} doubly_linked_list;

// O(n) time complexity
doubly_linked_list* find(doubly_linked_list* list, char* value) {
    doubly_linked_list* current = list;
    while (current != NULL) {
        if (current->value == value) {
            return current;
        }
        current = current->next;
    }
    return NULL;
}


// O(1) time complexity
void delete(doubly_linked_list* node) {
    node->prev->next = node->next;
    node->next->prev = node->prev;
    free(node);
}

void insert(struct doubly_linked_list* list, const char* value) {
    assert(list->next != NULL);
    struct doubly_linked_list* new_node = (doubly_linked_list*)malloc(sizeof(doubly_linked_list));
    if (new_node == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        exit(EXIT_FAILURE);
    }
    new_node->value = value;
    new_node->next = list->next;
    new_node->prev = list;
    list->next = new_node;
    new_node->next->prev = new_node;
}

int main(int argc, const char* argv[]) {
    doubly_linked_list* list = (doubly_linked_list*)malloc(sizeof(doubly_linked_list));
    list->next = (doubly_linked_list*)malloc(sizeof(doubly_linked_list));
    list->next->prev = list;
    insert(list, "Hello");
    insert(list, "World");
    insert(list, "!");
    doubly_linked_list* node = find(list, "World");
    delete(node);
    insert(list, "Goodbye");
    node = find(list, "Hello");
    delete(node);
    doubly_linked_list* current = list;
    while (current != NULL) {
        doubly_linked_list* next = current->next;
        if (current->value != NULL) {
            printf("%s\n", current->value);
        }
        // free(current);
        current = next;
    }
    return 0;   
}