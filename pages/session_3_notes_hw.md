- Chapter 5:
	- Hash Function is a function that takes an arbitarily long string of bits and produces a fixed size result.
	- given a message *m*, we can apply hash function *h* and sign on *h*(*m*) to save on computation
	- result of *h* is typically between 128 to 1024 bits as opposed to millions for the message *m* istself
	- hash functions are sometimes called *message digest* functions and the hash result is called the *digest* or fingerprint
	- 5.1 Security of Hash Functions:
		- Several requirements for a hash function:
			- must be a one way function: given *m* it should be easy to compute *h*(*m*) but given a value *x* it ios not possible to find *h*(*m*) = *x*
			- should be collision resistant: two differnt inputs *m_1* and *m_2* for which *h(m_1)* = *h(m_2)*
			- #+BEGIN_QUOTE
			  Definition 4: The Ideal hash function behaves like random mapping from all possible input values to the set of all possible output values
			  #+END_QUOTE
			- #+BEGIN_QUOTE
			  Definition 5: An attack on a hash function is a non-generic method of distinguishing the hash function from the ideal hash function
			  #+END_QUOTE
				- One generic attack: The birthday attack, which gens collisions. For a hash function with *n*-bit output, this requires about 2^*n*/2 steps (see exercise 5.3)
				- Cases where collision-resitance doesnt matter .Other situations we want a pre-image (given *x* find a *m* with *h*(*m*) = *x*)
					- #+BEGIN_QUOTE
					  From: Lessons From The History Of Attacks On Secure Hash Functions
					  A hash function is pre-image resistant if, given an output (image), an adversary can’t find any input (pre-image) which results in that output.
					  #+END_QUOTE
					- #+BEGIN_QUOTE
					  From: Lessons From The History Of Attacks On Secure Hash Functions
					  A hash function is second-pre-image resistant if, given one pre-image, an adversary can’t find any other pre-image which results in the same image.
					  #+END_QUOTE
					- Cases where we care most about second pre-image resistant
					- Requires about 2^*n* steps (where *n* is *n*-bit output)
					- We can also specifiy reduced bit-security like block ciphers.
						- #+BEGIN_EXAMPLE
						  512-bit hash function specifies a security level of 128 bits. In that case distinguishers are limited to 2^128 steps
						  #+END_EXAMPLE
						- #+BEGIN_QUOTE
						  From: Lessons From The History Of Attacks On Secure Hash Functions
						  The bottom line is that no widely-studied hash function has ever succumbed to a (second-)pre-image attack except for one.
						  #+END_QUOTE
						- Snefru which is vulnerable to Differental cryptanalysis
	- 5.2 Real hash functions:
		- 5.2.1 Simple but insecure Hash Function
			- Let *K* be a 256-bit key set to all zeroes. To hash the message *m*, first pad it in some way and break into 128-bit blocks *m_1*...*m_k*
			- Set *H_0* to a 128-bit block of all zeroes. Now we compute *H_i* = AES_*K*(*H_i-1*  XOR  *m_i*). Let *H_k* be the result of the hash function
				- Non-generic attack for this. Pick a message *m* such that after padding it splits *m_1* and *m_2*. Let *H_1* and *H_2* be the values computed inside of the hash's processing. *H_2* will also be the output (image)  of the hash f unction
				- Now let *m'_1* = *m_2* XOR *H_1* and *m'_2* = *H_2* XOR *m_2* XOR *H_1* and let *m'* be the message that splits into *m'_1* and *m'_2* after padding, *m'* also hashes to *H_2*
				- SO basically this is a collision when hashed with this hash function.
		- 5.2.2 MD5
			- MD5 is a 128 bit hash function, further development of MD4
			- Split the message into blocks of 512 bits. Last block is padded and the length of the message is included as well.
			- MD5 has a 128-bit state and split into four words of 32 bits each
			- TODO: md5 not really relevant to homework so im going to skip to SHA
		- 5.2.3 SHA-1
			- SHA-1 is a 160-bit hash function based on MD4.
			- 160-bit state consisting of five 32-bit words. SHA-1 uses linear reccurence to "strech" the 16 words of a message block to the 80 words it needs.
			- The linear reccurence ensures that each bit of message is used at least dozen times in the mixing function
			- Main problem is the 160-bit image size.
			- Collisions can be generated in only 2^80 steps
		- 5.2.4 SHA-224, SHA-256, SHA-384, SHA-512
			- so these basically have 224, 256, 384 and 512 images respectively.
			- designed to
			- be used with the 128, 192- 256 bit key sizes of AES as well as 112-bit key size of 3DES
			- Larger result = more security basically?
		- 5.3 Weakness of Hash Functions
			- 5.3.1 Length extensions
			- a message *m* is split into blocks *m_1*...*m_k* to the hash value *H*
			- then lets choose a message *m'* that splits into the block *m_1*...*m_k*, *m_k+1*.
			- the first *k*  blocks of *m'* are identical to the *k* blocks of message *m*, the hash *h*(*m*) is the intermidiate  value of computing *h*(*m"*)
			- therefore *h*(*m'*) = *h'*(*h*(*m*), *m_k+1*)
			- so therefore if we have *m'* we have to be very careful with padding and length field
			- need special procesing at the end of the hash functgion computation.
			- therefore computation of *h*(*m*) provides direct info about intermidate state after the *k* blocks of *m'*
			- distinguisher= canidate pairs of (*m*, *m'*) and check for this relationship
			- this is resolved in SHA-3
		- 5.3.2 Partial message collision
			- so, distinguishers:
				- specify the setting in  which it will differiante between hash function and ideal hash function
			- suppose we have a system that autheticates message *m* with *h*(*m* || X), where *X* is the authentication key
			- an attacker can choose a message *m* , but system will only authenticate a single message
			- the attacker can choose an iterative hash function.
				- Can choose two strings *m* and *m"*that lead to a collision when hashed by *h*. can be done in only 2^*n*/2 steps or so.
				- can get system to authenticate *m*, and then replace message with *m'*
				- so because *h* is computed iteratively, once there is a collision and rest of inputs are the same, hash value stays the same.
				- hashing *m'* leads to same value *h*(*m*||*X*) = *h*(*m'*||*X*). the same *m* and *m'* would work for all vcalues for *X*
		- 5.4 Fixing weaknesses
		-
-
-
-
-
-
- chapter 5 homework:
	- Exercise 5.3 Consider SHA-512-n, a hash function that first runs SHA-512 and then outputs only the first *n* bits of the result. Write a program that uses a birthday attack to find and output a collision on SHA-512-n, where n is a multiple of 8 between 8 and 48. Your program may use an existing cryptography library. Time how long your program takes when n is 16, averaged over five runs for each *n*. How long would you expect your program to take for SHA-512-256? For SHA-512?
-
-
-
-
-
- Extra reading notes:
	- Lessons From The History Of Attacks On Secure Hash Functions
		-
-
-
-
- Lecture:
	- SHA3 is here
	- Pedersen Hash (is cool apparently)
	- Assigned readings (which i didnt do yet has a big graph)
	- Hash functions are cool now, vs 2010
	- arbitarily large file reduces to image.
	- functions give out predictibly long output
	- repeadtly iterate functions until given output
	- Sponge construction: sequence of function applications , asorbing phase takes in message and reapply and reapply until random bytes are gone!!!
	- Blake construction is based on chacha (cha cha is cool apparently!)
	- Try out the ring libary yeah!
	- MACs:
		- authenticate! useful primative
		- Eve prevent tampering problem, straight forward
		- basically need a hash of a key and a message, MAC will return wheter its tampered or not
		- if eve has less to work with, less revealing of cryptography systems.
		- MACs dont actually reveal state of message, is a random mapping of Alice's message to output the repersents Alice's message
		- Random oracle same as hash functions
	- Length extension attacks:
		- Hmacs- prevent lentth extensions
		- but gMAC- not worried about length extensions
- *Need to figure symbolic reasoning and do the math for real!!!*
- we need AEADs to reach for i guess
- study groups