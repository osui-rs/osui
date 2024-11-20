#include <stdio.h>
#include <memory.h>
#include <libc.h>
#include <pthread.h>
#include <time.h>

extern boolean_t render(void* element);
extern void *event_loop(void* element);

void sleep_ms(long milliseconds)
{
    struct timespec ts;
    ts.tv_sec = milliseconds / 1000;
    ts.tv_nsec = (milliseconds % 1000) * 1000000;
    nanosleep(&ts, NULL);
}

boolean_t c_run(void* element)
{
    pthread_t thread;

    if (pthread_create(&thread, NULL, event_loop, element) != 0)
    {
        perror("Failed to create thread");
    }

    while (render(element))
    {
        sleep_ms(20);
    }
    return 0;
}