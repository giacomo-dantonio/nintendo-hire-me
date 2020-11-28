# Nintendo's "Hire Me" challenge

I will try to solve [Nintendo's *Hire Me* challenge](https://www.nerd.nintendo.com/files/HireMe) using Rust.

The challenge is written in C and consists in reverse engineering some pseudo-cryptographic function.

You are given a couple of arrays

    typedef unsigned char u8;
    typedef unsigned int u32;

    u8 confusion[512];
    u32 diffusion[32];

And a function

    void Forward(u8 c[32],u8 d[32],u8 s[512],u32 p[32]);

You can provide an array

    u8 input[32];

After the call `Forward(input,output,confusion,diffusion);` the
array `output` should contain the string `"Hire me!!!!!!!!"`.

    int main(int argc, char* argv[])
    {
        u8 target[]="Hire me!!!!!!!!";
        u8 output[32];

        Forward(input,output,confusion,diffusion);

        return memcmp(output,target,16); // => contact jobs(at)nerd.nintendo.com
    }

To solve the challenge you are required to code an algorithm which
will compute an `input` array for any given `target` string.

Nintendo distinguishes between three kinds of solutions:

    /*
    The solutions to this challenge belong to different levels :

    Level 1 : an iterative algorithm which typically takes more than a second to
    find a solution (for any given output). 

    Most people stop here, which is fine, but if you want to go further, there is :

    Level 2 : a non-iterative algorithm which typically takes less than a
    millisecond to find a solution (for any given output).

    Very few people have reached this level. But if you want to beat it completely,
    there's yet another castle...

    Level 3 : an algorithm which can provide any of the 2^128 solutions (for any
    given output).

    No-one has reached this level yet !
    */

I will attempt to solve the Level 2, but I will also think about how to generalize the algorithm to provide all the solutions.

## Tackling the challenge

[Step 1: Rewrite the challenge in Rust](doc/rewrite.md)

[Step 2: Analyze the forward function](doc/analyze.md)
