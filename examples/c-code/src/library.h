/**
Example C functions for testing Rust/C integration.

All functions start with `c_code_`.
*/

/*
These types are used in the functions below
*/

enum c_code_sample_enum_t {
    ITEM_ONE,
    ITEM_TWO,
    ITEM_THREE
};

struct c_code_data_t {
    char c;
    short s;
    int i;
    long l;
    float f;
    double d;
    enum c_code_sample_enum_t e;
};

/*
These integer `c_code_xxx_check` functions take a value and return `value + 1`.
*/

char c_code_char_check(char x);
signed char c_code_signed_char_check(signed char x);
unsigned char c_code_unsigned_char_check(unsigned char x);
short c_code_short_check(short x);
unsigned short c_code_unsigned_short_check(unsigned short x);
int c_code_int_check(int x);
unsigned int c_code_unsigned_int_check(unsigned int x);
long c_code_long_check(long x);
unsigned long c_code_unsigned_long_check(unsigned long x);

/*
This pointer check function takes a pointer to two ints, and returns a pointer to the second.

If passed NULL, it returns NULL.

If passed a pointer to one int, the result is undefined.
*/

int* c_code_int_ptr_check(int* p);

/*
These floating-point `c_code_xxx_check` functions take a value and return `value * 2.0`.
*/

float c_code_float_check(float x);
double c_code_double_check(double x);

/*
The enum check function converts ITEM_ONE to ITEM_TWO, ITEM_TWO to ITEM_THREE and ITEM_THREE to ITEM_ONE.
*/

enum c_code_sample_enum_t c_code_enum_rotate(enum c_code_sample_enum_t x);

/*
This struct check function adds one to every field in the structure, and returns a new structure.
*/



struct c_code_data_t c_code_struct_check(struct c_code_data_t x);

/*
This struct check function adds one to every field in the structure, by modifying the given structure.
*/

void c_code_struct_ref_check(struct c_code_data_t* p);

/*
End of file
*/
