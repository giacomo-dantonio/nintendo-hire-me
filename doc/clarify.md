# Step 3: inverting the `confuse` operator

Let's have a look at the `confuse` operator:

    static CONFUSION : [u8; 512];

    fn confuse(input: &[u8; 32]) -> [u8; 32]
    {
        let mut result = [0u8; 32];
        for j in 0 .. 32usize
        {
            result[j] = CONFUSION[input[j] as usize];
        }
        result
    }

`confuse` looks in the `CONFUSION` table for the entry at the index `input[j]`
and set it as `output[j]`.

The table `CONFUSION` contains `512` 8-bit entries. There are only `256`
possible distinct values of `u8`, so `CONFUSION` will certainly contain
repetitions. But, does it contain every possible `u8` value? If yes, it should
be easy to construct an inverse table.

Let's check it in a python shell.

    >>> len(set(CONFUSION))
    255
    >>> [x for x in range(0, 256) if x not in set(CONFUSION)]
    [250]

So the value `250` is missing. This means that we will not be able to invert
any string which contains the value `250`. But we will be able to invert all
the others.

    >>> FIRST_HALF = CONFUSION[:256]
    >>> SECOND_HALF = CONFUSION[256:]
    >>> [x for x in range(0, 256) if x not in set(FIRST_HALF)]
    [15, 17, 32, 62, 68, 90, 107, 117, 128, 158, 175, 177, 203, 213, 228, 250]
    >>> [x for x in range(0, 256) if x not in set(SECOND_HALF)]
    [2, 14, 21, 25, 100, 104, 115, 127, 135, 139, 144, 156, 225, 237, 246, 250]

Notice that `CONFUSION` consists of two subsequences of length `256` but those
sequences are missing a couple of `u8` value more. This will be an issue
when reverting `confuse_alot`.

I use python to generate an inverse map:

>>> def get_index(n):
...     try:
...             index = CONFUSION.index(n)
...             return "Some(%s)" % index
...     except:
...             return "None"
>>> indices = [get_index(n) for n in range(0, 256)]
>>> print(",\n".join(indices))
