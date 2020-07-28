#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  void (*callback)(uint32_t, void*);
  void *return_data;
} CallbackHandle;

void register_handler(const char *capability_name, CallbackHandle handle);

void start(const char *c_str_iface, uint32_t port);
