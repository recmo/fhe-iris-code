# FHE Iris Codes

**DO NOT USE IN PROD**

Experiments to see if iris codes can be matched in FHE with acceptable privacy and performance.

## Specification

For present purposes an iriscode consists of $\mathtt{data}$ bits and $\mathtt{mask}$, two $12\ 800$-bit bitvectors. When a mask bit is set the corresponding data bit should be ignored.

### Fractional hamming distance

A *fraction hamming distance* between two such iriscodes $\mathtt a$ and $\mathtt b$ is computed as

$$
\begin{aligned}
d &= \mathtt{a.data} ⊕ \mathtt{b.data} \\
\overline{m} &= \neg(\mathtt{a.mask} \vee \mathtt{b.mask}) \\
\mathrm{fhd}(\mathtt{a}, \mathtt{b}) &= \frac
{\mathtt{popcount}(d ∧ \overline{m})}
{\mathtt{popcount}(\overline{m})}
\end{aligned}
$$

where $d$ is a vector of data bits that are different, $\overline{m}$ is a vector of bits that are unmasked in $\mathtt a$ and $\mathtt b$ and $\mathtt{fhd}$ is the *fractional hamming distance*.

### Rotations

The $12\ 800$ bitvectors can be interpreted as $64 × 200$ bit matrices. We can then define a rotation as a permutation on the columns:

$$
\mathrm{rot}(\mathtt b, n)[i,j] = \mathtt b[i,(j+n)\ \mathrm{mod}\ 200]
$$

When applied to an iriscode this applies to $\mathtt{data}$ and $\mathtt{mask}$ equaly.

### Distance

The *distance* between two iriscodes $\mathtt a$ and $\mathtt b$ is defined as the minimum distance over rotations from $-15$ to $15$:

$$
\mathrm{dist}(\mathtt a, \mathtt b) = \min_{r∈[-15,15]}\ \mathrm{fhd}(\mathrm{rot}(\mathtt a, r), \mathtt b)
$$

### Uniqueness

To verify uniqueness we require that an iriscode $\mathtt a$ is a certain minimum distance from all previous iriscodes:

$$
\mathop{\Large ∀}\limits_{\mathtt b ∈ \mathtt{DB}}\ 
\mathrm{dist}(\mathtt a, \mathtt b) > \mathrm{threshold}
$$

where $\mathtt{DB}$ is the set of already registered iriscodes (currently 3 million entries).

When there is a match, we are also interested in finding the location of the best match. Both can be addressed by implementing a method that returns the index of the minimum distance entry.

## FHE specific notes

### Minimal proposal

Keep the query $\mathtt{a}$ and the $\mathtt{mask}$ s cleartext, but $\mathtt{DB}$ entries ciphertext.

In cleartext compute a rotation of $\mathtt{a}$ and

$$
\begin{aligned}
\overline{m} &= \neg (\mathtt{a.mask} ∨ \mathtt{b.mask}) \\
M &= \mathrm{popcount}(\overline{m}) \\
\end{aligned}
$$

Then in cyphertext compute

$$
p = \mathtt{popcount}((\mathtt{a.data} ⊕ \mathtt{b.data}) ∧ \overline{m})
$$

Then in ciphertext maintain a current lowest distance $\frac{p'}{M'}$ and compare

$$
p ⋅ M' < p' ⋅ M 
$$

### Better proposal

When creating an iriscode, produce all 31 rotations in ciphertext and do the comparisons with both $\mathtt{a.data}$ and $\mathtt{b.data}$ in ciphertext.

### Better proposal

Also keep $\mathtt{mask}$ s in ciphertext.
