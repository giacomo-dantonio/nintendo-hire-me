# Step 2: Analyze the `forward` function

I start with a small refactoring to make the code more readable.
I'll rename the function parameters and remove the constant arrays from the
parameter list:

    fn forward(input: &mut [u8; 32], output: &mut[u8; 32])
    {
        for _ in 0 .. 256usize
        {
            for j in 0 .. 32usize
            {
                output[j] = CONFUSION[input[j] as usize];
                input[j] = 0;
            }

            for j in 0 .. 32usize
            {
                for k in 0 .. 32usize
                {
                    input[j] ^= output[k] * (((DIFFUSION[j] >> k) & 1) as u8);
                }
            }
        }

        for i in 0 .. 16usize
        {
            output[i] = CONFUSION[input[i*2] as usize]^CONFUSION[input[i*2+1] as usize + 256];
        }
    }

At this point we can see that the function consists of three steps:

1. Populate the `output` array using `input` as indices for the table `CONFUSION`. `input` is set to zero after this step.
2. Apply some kind of convolution operator to `output` and write the outcome to `input`.
3. Populate the `output` array similar to step 1, but in a slightly more complicate way.

The first two step are run `256` times, the last step only once.

## Refactor `forward`

Let's refactor the code to make the three steps explicit.

    fn confuse(input: &[u8; 32]) -> [u8; 32]
    {
        let mut result = [0u8; 32];
        for j in 0 .. 32usize
        {
            result[j] = CONFUSION[input[j] as usize];
        }
        result
    }

    // ACHTUNG! input here is output in forward
    fn convolute(input: &[u8; 32]) -> [u8; 32]
    {
        let mut result = [0u8; 32];

        for j in 0 .. 32usize
        {
            for k in 0 .. 32usize
            {
                result[j] ^= input[k] * (((DIFFUSION[j] >> k) & 1) as u8);
            }
        }

        result
    }

    fn confuse_alot(input: &[u8; 32]) -> [u8; 32]
    {
        let mut result = [0u8; 32];

        for i in 0 .. 16usize
        {
            result[i] = CONFUSION[input[i*2] as usize]^CONFUSION[input[i*2+1] as usize + 256];
        }

        result
    }

    pub fn forward(input: &mut [u8; 32]) -> [u8; 32]
    {
        for _ in 0 .. 256usize
        {
            let tmp = confuse(input);
            *input = convolute(&tmp);
        }

        confuse_alot(&input)
    }

I aldo added a unit test, to make sure I didn't break anything while refactoring.

## Solution strategy

So what forward is doing is just

    forward(input) = confuse_alot(convolute(confuse(convolute(confuse(...(input))))))

Now I have a strategy. I will try to invert the tree three steps
`confuse`, `convolute` and `confuse_alot`. That is, I will write three functions
`clarify` (the opposite of confuse), `deconvolute` and `enlighten` such that

    confuse(clarify(input)) == input
    convolute(deconvolute(input)) == input
    confuse_alot(enlighten(input)) == input

Once I have those I can set

    input = clarify(deconvolute(...(clarify(deconvolute(enlighten(target))))))

The operations will then all cancel sequentially givin

    forward(input) = target
