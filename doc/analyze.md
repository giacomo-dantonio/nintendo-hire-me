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
    fn convolution(input: &[u8; 32]) -> [u8; 32]
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
            *input = convolution(&tmp);
        }

        confuse_alot(&input)
    }
