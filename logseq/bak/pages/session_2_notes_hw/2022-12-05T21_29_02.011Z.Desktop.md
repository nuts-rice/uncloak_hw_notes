- Notes
	- Current generation (2010) of block ciphers is a block size of 128 bits
	- block ciphers are kinda a very big key-dependent table (in a conceptual type of way)
		- for any fixed key, you compute a lookup table that maps plaintext to cipher text
		- no two entries of the table are the same (otherwise this would be a collision).
		- this is pretty much a permutation: a list of all possible elements where order is different.
		- A block size of k bits specifies the permutations on k-bit values for each of the key vals.
		- takes 2^k possible k-bit inputs and maps each to an unique k-bit output
	- Attacks:
		- Related key attack sounds like making an isomorphism between keys
		- "changes the key for every message by incrementing by one" sounds like bit shifting
		- make sure to cover any type of attack in building cryptography!
	- Defn of block cipher security
		- #+BEGIN_QUOTE
		  Definition 1: A secure block cipher is one for which no attack exists
		  #+END_QUOTE
		- #+BEGIN_QUOTE
		  Definition 2: An attack on a block cipher is a non-generic method of distinguishing the block cipher from an ideal block cipher
		  #+END_QUOTE
			- Given block cipher X, compare it to an ideal block cipher with same block size and key size
				- use a *distinguisher* which is an algo that is given a black box func that computes either X or an ideal block cipher
					- the distinguisher should figure out wheter the black box func implements the block cipher X or the ideal cipher
						- Shouldnt be generic/trivial to make an attack. Shouldn't be a similar distinguisher for *any* block cipher
						- like a test case: so basically we assert that an executed block cipher X is *expected* to have a value *t* occur only once, compare to the ideal block cipher
		- Parity of a permutation
			- encryption under single key corrosponds to lookup in permutation table, which we can construct in two steps
				- Initialize the table with the identity mapping giving element at index *i* the value *v*.
				- then create the permutation thet we want by repeadtly swapping two elements in the table.
					- two types of permutations: those that can be constructed from an even number of swaps (even pemutations) and those with odd number of permutations (odd permutations)
					- using 128 bit block size working on 32 bit words. it's hard to build odd permutations from these small number of operations, so virtually all block ciphers have even permutations
				- So this is a distinguisher: the parity attack. If we expect permutations to be odd, that would be the ideal block cipher
			- #+BEGIN_QUOTE
			  Definition 3: An ideal block cipher implements an indepently chosen random even permutation of each of the key values 
			  #+END_QUOTE
				- But in practice parity attacks are a more formal definition of security to be achieved, so we really should consider the superset of realistic attacks as opposed to unrealistic ones. then start with realistic and find new ones
				-
	- DES:
		- XOR: circle plus operator: bitwise addition
		- Has complement property: which is not *ideal*
			- If you complement L and R at the beginning of the round and complement *K_i* thhen output will be complement as well. This propogates throughout whole of the cipher
			- this has led to attacks to be formulated!
	- AES:
		- Plaintext comes in as 16 bytes (128 bits)
			- FIrst operation is to XOR plaintext with 16 bytes of round key
			- ![Screenshot from 2022-12-02 12-36-37.png](../assets/Screenshot_from_2022-12-02_12-36-37_1670002622558_0.png)
				- XOR adds key material to the data, S-boxes provide nonlinearility, and byte shuffle and mixing provides diffusion
	- Chapter 4:
		- Padding:
			- so blocvk chain modes require a bit of padding to be the exact multiple of the block size we need
			- very simple is that we can add 0s. This is a bad idea as its not tractable (reversable) as plaintext p and p||0 (concated with 0s) is the same padded form
			- How do we do it then?
				- Let *P* be the plaintext and *l(p)* be length of plaintext in bytes, *b* will be block size of the cipher in bytes.
					- One of two schemes we can use:
						- Append a single byte with value of 128, then as many zero bytes as required to make overall length a multiple of b. Num of zeroes added is the range 0...b-1
						- Determine the number of padding bytes required. This is the number *n* which satisies 1<=n<=b and n +l(p) is a multiple of *b*
					- Any padding scheme is fine, as long as its tractable.
		- ECB:
			- Simplest method is the electronic codebook mode, this is defined by ![image.png](../assets/image_1670204330698_0.png)
			- We dont use ECB lol
		- CBC:
			- Cipher block chaining is widely used, problems of ECB are avoided by XORing each plaintext block with previous ciphertext block.
			- ![image.png](../assets/image_1670204484377_0.png)
			- randomized plaintext using previous ciphertext block.
			- so how are we supposed to use for C_0, this is known as the initialization vector or *IV*
		- Nonce-generated IV
			- we use an unique number named the *nonce*
			- typically this is message number combined with some other information
			- the IV necesarry for CBC is generated by encrytping nonce
				- 1. Assign a message number to the message. Typically the message number is provided by a counter starting at 0.
				  2. use the message number to construct a unique nonce. For a given key, nonce should be unique to the whole sytstem, not the entire system. 
				  
				  #+BEGIN_EXAMPLE
				  if the same key is used to encrypt traffic in two directions, the nonce should consist of the message number plus indication of direction this message is being sent in
				  #+END_EXAMPLE
				- 3. Encrypt the nonce with the block cipher to generate the IV
				- 4. Encrypt the message in CBC using this IV.
				- 5. Add enough information to the ciphertext to ensure the reciever can reconstruct the nonce.
	- Chances of a collison:
		- Lets say we encrypt *M* blocks in total. A good estimate of *M*(*M* -1)/2 pairs of blocks and each pair has a chance of 2^-*n* of being equal, where *n* is the block size of the block cipher. So the expected number of equal ciphertext blocks is the *M*(*M* - 1)/2^*n*+1 which is close to unity when M = 2^*n*
		-
		-
- Homework
	- 1; How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?
		- 140 million TB lol (pg 44)?? Nah it could actually be pg 46
		- Keyspace and cipherspace exponiation and add them
	- 5; Suppose you have a processor that can perform a single DES encryption or decryption operation in 2^-26 seconds. Suppose you also have a large number of plaintext-ciphertext pairs for DES under a single unknown key. How many hours would it take, on average, to find that DES key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of 2^14 processors?
		- (Refer to classical introduction to cryptography)
		- ![Screenshot from 2022-12-01 12-15-54.png](../assets/Screenshot_from_2022-12-01_12-15-54_1669914971468_0.png)
		- exhaustive search of half of keyspace requires 2^n-1 work so:
			- 2^26 keys per second, go through 2^56 key canidates = 2^56 / 2^26 = 298261.618 hours / 2 for average case of half of keyspace =  149130.809 hours
	- Consider a new block cipher, DES2, that consists only of two rounds of the DES block cipher. DES2 has the same block and key size as DES. For this question you should consider the DES 
	   function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for DES2 under a single, unknown key. Given an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit DES key. Can your algorithm be converted into a distinguishable attack against DES2?
		- Can obtain intermidiate values (l_1, r_1, l_2, r_2) for DES2
			- TODO Formalize here
		- But also there a more efficient probablistic attack using a lookup table, and caching plaintext and compare it to message pairs
			- TODO  Formalize here
			-
	- 8; Familiarize yourself with a cryptographic CLI tools. A popular open source package is OpenSSL. Using an existing cryptographic library, decrypt the following ciphertext (in hex)
	- #+BEGIN_EXAMPLE
	  53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07
	  #+END_EXAMPLE
	- with the following 256-bit key (also in hex):
	- #+BEGIN_EXAMPLE
	  80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
	  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
	  #+END_EXAMPLE
	- using AES
	- 9; Using an existing cryptography library, encrypt the following plaintext (in hex)
	- #+BEGIN_EXAMPLE
	  29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D
	  #+END_EXAMPLE
		-
		- TODO finish and post here, maybe use a bash script (xxd and openssl cli)
	- with the following 256-bit key from problem 8, using AES. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.
-
	- 10; Write a program that experimentally demonstrates the complementation property for DES. This program should take as input a key *K* and a plaintext 
	   and demonstrate that the DES complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.
		-
	- 1; Let  be a plaintext and let  be the length of  in bytes. Let  be the block size of the block cipher in bytes. Explain why the following is not a good padding scheme:
	- Suppose you, as an attacker, observe the following 32-byte ciphertext *C* (in hex)
	- #+BEGIN_EXAMPLE
	  46 64 DC 06 97 BB FE 69 33 07 15 07 9B A6 C2 3D
	  2B 84 DE 4F 90 8D 7D 34 AA CE 96 8B 64 F3 DF 75
	  #+END_EXAMPLE
	- and the following 32-byte ciphertext *C'* (also in hex)
	- #+BEGIN_EXAMPLE
	  51 7E CC 05 C3 BD EA 3B 33 57 0E 1B D8 97 D5 30
	  7B D0 91 6B 8D 82 6B 35 B7 8B BB 8D 74 E2 C7 3B
	  #+END_EXAMPLE
	- Suppose you know these ciphertexts were generated using CTR mode with the same nonce. The nonce is implicit, so it is not included in the ciphertext. You also know that the plaintext *P* corresponding to  *C* is
	- #+BEGIN_EXAMPLE
	  43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79
	  70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F
	  #+END_EXAMPLE
	- What information, if any, can you infer about the plaintext *P'* corresponding to *C'*?
		- XOR isn't an one time pad here, properties of XOR
		- TODO Formalize here
	- TODO add more of the questions yeah
- Lecture notes:
	- TODO I missed a couple minutes of the lecture after discussion of the homework soloutions. Review the video
	- ECB penguin?
	- Stream cipher constructors are cool!
	- Padding attacks *look it up?* CBC mode (variations on chosen ciphertext modes)
		- Have an oracle call for if block is padded correctly
	- well, which block mode should we use?
		- Actually we're more realistically going to use AEAD (Authenticated encryption with assosiated data)
			- TLS library for Message Authenitican Codes and