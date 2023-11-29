#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

char *summarize_text(const char *input);

void free_memory(char *ptr);

} // extern "C"
