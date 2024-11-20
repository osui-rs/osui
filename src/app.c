#include <stdio.h>
#include <memory.h>
#include <libc.h>
#include <pthread.h>
#include <time.h>

typedef struct
{
    uint32_t* state;
    void* element;
} EventArgs;

extern void render(void* element, uint32_t state);
extern void event(void* element, uint32_t* state);
extern void init_event(void* element, uint32_t* state);

void sleep_ms(long milliseconds)
{
    struct timespec ts;
    ts.tv_sec = milliseconds / 1000;
    ts.tv_nsec = (milliseconds % 1000) * 1000000;
    nanosleep(&ts, NULL);
}

void *event_checker(void *args)
{
    EventArgs* event_args = (EventArgs*)args;
    init_event(event_args->element, event_args->state);
    while (1) {
        if (event_args->state == 0) { return NULL; }
        event(event_args->element, event_args->state);
    }
    return NULL;
}

void *command_checker(void *args)
{
    EventArgs* event_args = (EventArgs*)args;
    init_event(event_args->element, event_args->state);
    while (1) {
        if (event_args->state == 0) { return NULL; }
        event(event_args->element, event_args->state);
    }
    return NULL;
}

boolean_t c_run(void* element)
{
    pthread_t thread;
    uint32_t state = 3;
    EventArgs event_args = { .element = element, .state = &state };

    if (pthread_create(&thread, NULL, event_checker, &event_args) != 0)
    {
        perror("Failed to create thread");
    }

    while (1)
    {
        if (state == 0) {
            free(element);
            return 0;
        } else if (state == 1) {
            free(element);
            return 1;
        }
        render(element, state);
        sleep_ms(20);
    }
}