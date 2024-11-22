#include <stdio.h>
#include <memory.h>
#include <libc.h>
#include <pthread.h>
#include <time.h>

typedef struct {
    void* element;
    void* sender;
    void* receiver;
    boolean_t running;
} LArgs;

extern boolean_t render(void* element);
extern void *event_loop(void* element);
extern void *cmd_loop(void* element);

void sleep_ms(long milliseconds)
{
    struct timespec ts;
    ts.tv_sec = milliseconds / 1000;
    ts.tv_nsec = (milliseconds % 1000) * 1000000;
    nanosleep(&ts, NULL);
}

boolean_t c_run(void* element, void* sender, void* receiver)
{
    pthread_t event_thread;
    pthread_t cmd_thread;
    LArgs args = { .element = element, .sender = sender, .receiver = receiver, .running = 1 };

    if (pthread_create(&event_thread, NULL, event_loop, &args) != 0)
    {
        perror("Failed to create thread");
    }

    if (pthread_create(&cmd_thread, NULL, cmd_loop, &args) != 0)
    {
        perror("Failed to create thread");
    }

    while (render(element) && args.running)
    {
        sleep_ms(20);
    }
    return 0;
}