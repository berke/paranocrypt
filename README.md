# PARANOCRYPT

## What is this?

PARANOCRYPT is a ridiculous, over-the-top authenticated multi-cipher
symmetrical encryption mode, based on the following assumptions:
- Hardware contains hidden key recorders that sample some or all the
used encryption keys in internal storage.  These are triggered by the
use of hardware-accelerated encryption primitives (e.g. AESNI), but
could also be triggered by keyschedule detectors.
- The captured keys are stored in non-volatile memory and/or
transmitted through side-channels e.g. WiFi, BT or Ethernet
subchannels, modulated emissions or using steganography.
- Some encryption algorithms are simply broken (by design), which
means that decryption parameters (not necessarily the original key)
can be recovered from a very small amount of traffic (one
plaintext-ciphertext pair, or a few ciphertext pairs).
- These are available to multiple eavesdroppers, they don't all have
access to the same channels.  For example, the NSA might have an
efficient AES breaker, some PRC entity might have access to some
hidden side-channels, all of them be able to recover intentionally
broadcast key material.

More importantly, it is assumed that symmetric encryption using
conventional ciphers and modes can be broken given sufficient traffic.

More formally, let N be some measure of complexity of the encryption
function f(K,P) = C where K is the key, P is the plaintext and C is
the ciphertext.  For example N could be some measure of circuit complexity.

Let T be the number of available plaintext-ciphertext pairs.  We
suppose the attacker(s) have access to a machine that can break
ciphers with effort g(N,T).  We don't specify units for g, but say
values of g of 2^40 are feasible (the attacker can obtain the key in a
reasonable time) and values larger than 2^80 are not.

For small T, g(N,T) is prohibitively large, but decreases rapidly
and becomes feasible beyond T = h(N).

For example, a typical AES128 block cipher can be represented
using a few tens of thousands of gates, let's say N ~ 20k gates.
When T is small (say less than ten) we hypothesize that
g(N,T) is very large, on the order of 2^256, but above, say
(and this is purely speculative) T=1000 g(N,T) collapses
(with the proper algorithm and machine) to a feasible number
such as 2^40.

Nothing is known about h(N).  In principle, the number U of unknowns
is simply the key size, which is a constant.  We've assumed that one
circuit that checks one traffic pair (T=1) has size N.  Each gate will
introduce a few intermediate unknowns and equations, say A unknowns
and B equations.  Overall we will have N×B equations and N×A unknowns.

If we increase T, a circuit smaller than T×N is possible by using
multiplexers etc.  As traffic introduces more equations than unknowns,
this can keep the number of unknowns from increasing faster than the
number of equations.

We hypothesize that at some point, given sufficient T, the system will
be severely overconstrained, and will become solvable, given an
adequate algorithm (assumed to exist).

Remember, all of this is speculative, this is not a mathematical
demonstration, but a hand-waving algorithm.  We make no claims about
the existence of such an algorithm.

Nevertheless, we could hypothesize that h(N) doesn't grow too fast.

Basic linear algebra routines (such as Gaussian elimination or QR factorization)
be cubic in the number of variables.  Under good conditions, iterative non-linear
solvers (Newton etc.) have superlinear convergence and require a very small
number of iterations (maybe a dozen), but the steps require solving linear systems.
Thus for general non-linear (continuous) problems cubic or quartic complexities
are often seen.

To be conservative, we have to assume that the attackers have an algorithm that 
has low complexity.  I believe quadratic complexity is a reasonable assumption.

TLS/SSL started seeing widespread, systematic usage in the late 2000s.  These
days, multi-megabyte webpages are the norm, but back then I would say that 100 kB
would be a reasonable estimate of the amount of data fetched by a browser to
display a "typical" web page.  Let's round that to 2¹⁷ (131072) bytes,
which is 2¹⁰ = 1024 128-byte blocks.  Let's say the powers-that-be allowed
widespread usage of AES because T₀ = 1024 is sufficient to break AES128,
which we assumed to be representable using N₀=20k gates.
Hence h(N₀) = T₀.  Of course they could just as well be attacking
the key exchange.  But they also need to be able to decrypt say
BitLocker or LUKS volumes which do not use asymmetric encryption.

Thus we will assume that c is approximately 2.6e-6 so that h(N₀) = c N₀² = T₀.

Therefore our encryption mode should change keys every T₀ blocks, or more often.
But what does changing keys mean?  If we have a key derivation function, that
function will be subject to a similar analysis.

Suppose we use a very complex key derivation scheme (such as Argon2) that
requires a very large number N₁ of gates to represent.  For example N₁ = 10⁹
is feasible.  Then using the same model h(N₁) will be 2.6×10¹² which would
be about 333 petabytes.

## Summary

Thus our model is as follows.  Suppose you have a secret key K.
The more you use it, the more it gets worn out.  The simpler your
algorithm (as counted by the number of gates) the faster it wears out.

The solution is then simple.

We have two algorithms, an expensive key derivation function D
and a block cipher f.  The key derivation is good for S uses (say S = 10000)
and f is good for T uses (say T = 100).

Start with a master key K₀.

Every time we want to encrypt something, select a random number R₁.
Compute K₁=D(K₀,R₁).  By hypothesis, we are allowed to do that S times.
In other words, because D is expensive, K₀ wears out only a little and
we can re-use K₀ 10000 times.

Now we have an intermediate key K₁.
Let's pick a second random number R2 and compute K₂=D(K₁,R2).

We proceed that way n times until we get Kₙ.  We can now use Kₙ
up to T times, that is for our first T plaintext blocks
X[1],...,X[T].

If we are using a stateful ciphering mode, let q[0] be the initial
state (usually called the nonce.)  We send R₁[1],...,Rₙ[1],q[0],
f(Kₙ[1],q[0],X[1]),f(Kₙ[1],q₁,X[1]),...,f(Kₙ[1],q[T],X[T]).

Then Kₙ[1] is spent, and we need to select a new Rn
and derive a new Kₙ[2].

We do that until K[n-1] is spent as well, then we have
to select new K[n-1] and K[n], and so on and so forth.

## Multi-cipher

We have said that maybe some of the popular algorithms are completely
broken, that is, they have a very small T₀ value, i.e. very little
traffic is required to break them.

Or maybe some algorithms are "trapped", the hardware detects their
usage and records or secretly embeds or transmits cryptomaterial.

Thus we need to combine multiple ciphers by applying them in series.
These ciphers should include:
- Best-in-class "government" algorithms from multiple, opposing
countries.  Today main geopolitical power blocks that conduct public
crypto research are Western/NATO, China and Russia.  Thus we should
use AES256, GOST and some Chinese cipher.
- Best-in-class "civilian" algorithms, such as Chacha, Blowfish and IDEA.
- A custom, improvised cipher, for example obtained using a Feistel
construction with an existing or modified hash function.

Of course these ciphers should use independent keys, so that if
one of them is compromised (in the sense that it somehow "emanates"
key material) it won't compromise the other ones.

## RAM footprint and other countermeasures

We have been assuming that the hardware is compromised to some extent.
If it is completely compromised all of this is pointless.  We will
therefore assume that it is partially compromised.  The spying circuitry
has limited capabilities that have been fine-tuned for the most common
use cases (such as TLS, SSH, BitLocker, LUKS...)  Their storage capacity
and/or transmission bandwidths are necessarily limited, as they would
otherwise put an unacceptable price or performance penalty.
Maybe modern computers have a 1 GB flash chip that records suspected
crypto variables.  Maybe it's 16 GB.  Maybe keyboard controllers have
a built-in recorder that logs all key presses.  Displays can probably
be read over compromising emanations, and maybe they also record
some of the displayed information.

The storage and analysis capacity being necessarily limited, we have to
make the crypto operations expensive enough that they will require a
large fraction of the computer's capacity.

In addition we need to define mechanisms that defeat screen, key and
cryptologgers.

Cryptologgers are defeated by using a large number of keys,
multiple ciphersa and encryption that requires substantial RAM.

The processed information (plaintext) is displayed on the screen, so
the screen is a weak link.

However if a modern monitor that uses eDP or HDMI with DRM is used,
TEMPEST screen surveillance will require expensive, proprietary
receivers in physically close proximity.

The video controller or screen might still take screenshots,
but as it necessarily has limited memory and/or covert transmission
bandwidth it should be possible to overwhelm it.

Note that it is probably hopeless to try to defeat targeted government
surveillance unless one uses carefully vetted hardware in an
EMSEC-hardened room, but evading mass surveillance seems possible.

Here is a key entry method that could work.

The entry program displays a grid of continuously changing
substitution tables.

The user has two input devices, say a keyboard and a mouse,
and two output devices, the screen and the sound output.

The mouse pointer is hidden.  By moving the mouse, the user selects a
cell of the screen grid.  The indices of the selected cell are read
out over the audio channel.  The user then maps the input character to
the output character by looking at the selected cell and presses the
corresponding key.  This changes all the cells.  The cells also
continously change by themselves.  In other words the user moves the
mouse randomly until a cell is identified.  Then the user selects a
key to press and waits until the correspoding key displays the desired
entry.

The input program forces the user to make random choices.

Even when the cell contents do not change, the display changes
continuously.  Maybe there is an animated background.  The fonts,
colors and letter positions change continuously.  The letters
and colors are selected so that they are hard to capture
on camera and so that they are lost in compression artefacts
unless a very high-bandwidth codec is used.

Spying on the user requires simultaneously spying on the mouse,
the keyboard and the screen, and requires accurate synchronization.

As there will be many input errors, redundancy is introduced
to help the user.

##

K0

R1=RND()
K1=D(R1,K0)
R2=RND()
K2=D(R2,K1)
R3=RND()
K3=D(R3,K2)
Y=F(K3,X)
M=[R1,R2,R3,Y]
