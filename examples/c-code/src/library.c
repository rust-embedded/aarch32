/**
Example C functions for testing Rust/C integration.
*/

#include "library.h"

char c_code_char_check(char x) {
    return x + 1;
}

signed char c_code_signed_char_check(signed char x) {
    return x + 1;
}

unsigned char c_code_unsigned_char_check(unsigned char x) {
    return x + 1;
}

short c_code_short_check(short x) {
    return x + 1;
}

unsigned short c_code_unsigned_short_check(unsigned short x) {
    return x + 1;
}

int c_code_int_check(int x) {
    return x + 1;
}

unsigned int c_code_unsigned_int_check(unsigned int x) {
    return x + 1;
}

long c_code_long_check(long x) {
    return x + 1;
}

unsigned long c_code_unsigned_long_check(unsigned long x) {
    return x + 1;
}

int* c_code_int_ptr_check(int* p) {
    return p + 1;
}

float c_code_float_check(float x) {
    return x * 2.0;
}

double c_code_double_check(double x) {
    return x * 2.0;
}

/*
The enum check function converts ITEM_ONE to ITEM_TWO, ITEM_TWO to ITEM_THREE and ITEM_THREE to ITEM_ONE.
*/

enum c_code_sample_enum_t c_code_enum_rotate(enum c_code_sample_enum_t x) {
    if (x == ITEM_ONE) {
        return ITEM_TWO;
    } else if (x == ITEM_TWO) {
        return ITEM_THREE;
    } else {
        return ITEM_ONE;
    }
}

struct c_code_data_t c_code_struct_check(struct c_code_data_t x) {
    struct c_code_data_t result = {
        .c = c_code_char_check(x.c),
        .s = c_code_short_check(x.s),
        .i = c_code_int_check(x.i),
        .l = c_code_long_check(x.l),
        .f = c_code_float_check(x.f),
        .d = c_code_double_check(x.d),
        .e = c_code_enum_rotate(x.e),
    };
    return result;
}

/*
This struct check function adds one to every field in the structure, by modifying the given structure.
*/

void c_code_struct_ref_check(struct c_code_data_t* p) {
    p->c = c_code_char_check(p->c);
    p->s = c_code_short_check(p->s);
    p->i = c_code_int_check(p->i);
    p->l = c_code_long_check(p->l);
    p->f = c_code_float_check(p->f);
    p->d = c_code_double_check(p->d);
    p->e = c_code_enum_rotate(p->e);
}

/*
End of file
*/
