#include <stdio.h>
#include <memory.h>
#include <libc.h>
#include <pthread.h>
#include <time.h>

extern boolean_t render_loop(void* args);
extern void *event_loop(void* args);
extern void *cmd_loop(void* args);

boolean_t c_run(void* args)
{
    pthread_t event_thread;
    pthread_t cmd_thread;

    if (pthread_create(&event_thread, NULL, event_loop, args) != 0)
    {
        perror("Failed to create thread");
    }

    if (pthread_create(&cmd_thread, NULL, cmd_loop, args) != 0)
    {
        perror("Failed to create thread");
    }

    return render_loop(args);
}