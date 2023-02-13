- 2.1
	- ![course-2023-02-10 Session 11 Notes-1676049266999.jpeg](https://publish-01.obsidian.md/access/27ad70359043eae211d68e0c184ee8b6/media/image/course-2023-02-10%20Session%2011%20Notes-1676049266999.jpeg)
	- #+BEGIN_QUOTE
	  Defn: *one way function*: easy to compute but hard to inverse is difficult to hard
	  #+END_QUOTE
	- #+BEGIN_QUOTE
	  Defn: *trapdoor*: auxillary information that allows inverse to be computed easily
	  #+END_QUOTE
	- Decryption function *d_kpriv* is easily computable with private key *k_priv* but is difficult to compute with someone with only public key *k_pub*
	- *k_priv* would be the *trapdoor information* for the encryption function *e_kpub* because without it, would be very difficult to compute the inverse of *e_kpub*
	- A proof of if one-way functions exist (can be verified) would solve *P = NP* problem lol
	- so basically we assume cryptosystems will fail to be disproven to run at X time and explain or generate unexpected behavior...like models of physics
	- Some one-way functions proposed by diffie-hellman:
		- knapsack problem
		- exponiation mod *q*
	- public key method described by diffie hellman called the *Diffie-Hellman key xchange*
		- based on discrete log problem being difficult to solve
- 2.2 Discrete Log problem
	- The Diffie Hellman public key construction is based on discrete lograthm problem in finite field *F_p* (field with prime number of elements) or *Z/pZ*
	- #+BEGIN_IMPORTANT
	  Let *p* be a large prime 
	  #+END_IMPORTANT