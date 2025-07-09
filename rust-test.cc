
#define MALLOC_QUOTA 0x100000
#define TEST_NAME "RUST"
#include "cheri.h"
#include "cheri.hh"
#include "fail-simulator-on-error.h"
#include "tests.hh"
#include <cstdlib>

/* Imports from Rust */
extern "C" int                zero();
extern "C" int                add(int a, int b);
extern "C" long long unsigned div(long long unsigned a, long long unsigned b);
extern "C" void               arith_tour();

extern "C" void *animal_make();
extern "C" void  animal_speak(void *);
extern "C" void  animal_destroy(void *);
extern "C" void  zoo_tour();

extern "C" void libcall_tour();

/* Things that Rust expects from us */
extern "C" void cheriot_print_str(char *s)
{
	printf("%s", s);
}

extern "C" void *cheriot_alloc(size_t size)
{
	// debug_log("Trying to allocate {} bytes!", size);
	Timeout timeout{5};
	void   *ret = heap_allocate(&timeout, MALLOC_CAPABILITY, size);
	TEST(CHERI::Capability{ret}.is_valid(),
	     "Allocation is invalid, got pointer: {} -- {}",
	     ret,
	     (int)ret);
	return ret;
}

extern "C" void cheriot_free(void *ptr)
{
	free(ptr);
}

extern "C" void cheriot_panic()
{
	debug_log("Reached panic!");
}

unsigned short lfsr = 0xACE1u;
unsigned       bit;

extern "C" char cheriot_random_byte()
{
	bit         = ((lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5)) & 1;
	return lfsr = (lfsr >> 1) | (bit << 15);
}

int test_rust()
{
	int zero_from_rust = zero();
	debug_log("Got zero from rust: {}", zero_from_rust);

	int add_from_rust = add(4, 2);
	debug_log("Got add from rust: {}", add_from_rust);

	long long unsigned div_from_rust = div(4, 2);
	debug_log("Got div from rust: {}", div_from_rust);

	debug_log(
	  "Until you see 'Back to C', all messages are directly from Rust.");
	arith_tour();
	debug_log("Back to C.");

	debug_log("Making an animal from rust..");
	void *animal = animal_make();
	debug_log("Got it: {}", animal);
	debug_log("Let's make it speak!");
	animal_speak(animal);
	animal_destroy(animal);

	debug_log("Making an second animal from rust..");
	void *animal2 = animal_make();
	debug_log("Got it: {}", animal2);
	debug_log("Let's make it speak!");
	animal_speak(animal2);
	animal_destroy(animal2);

	debug_log(
	  "Until you see 'Back to C', all messages are directly from Rust.");
	zoo_tour();
	debug_log("Back to C.");

	debug_log(
	  "Until you see 'Back to C', all messages are directly from Rust.");
	libcall_tour();
	debug_log("Back to C.");

	debug_log("All done.");

	return 0;
}
