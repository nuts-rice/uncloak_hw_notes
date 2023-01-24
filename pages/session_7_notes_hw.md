- # Book notes:
-
- Chapter 9:
-
- chapter 10:
- Primes
	- 10.1 Divisibility and primes
		- Primes! no divisiors but itself and 1
		- any interger not prime and greater than 1 is *composite*
		- #+BEGIN_EXAMPLE
		  Lemma 1: IF A /B and b/c then a / c
		  #+END_EXAMPLE
		- #+BEGIN_EXAMPLE
		  Lemma 2: Let n be a positive number greater than 1. Let d be the smallest divisor of  n that is greater than 1. Then d is prime
		  #+END_EXAMPLE
		- we assume it is not true and show that the assumption leads to a contraditction.
		- #+BEGIN_EXAMPLE
		  Euclid: There are infinite number of primes
		  #+END_EXAMPLE
		- #+BEGIN_QUOTE
		  fundemental theorem of arithimatic: any interger greater than 1 can be written in exactly one way as the product of primes
		  #+END_QUOTE
	- 10.2 generating small primes
		- Sieve of Erasothenes
			- ``` 
			  function SmallPrimeList
			  input n limit on primes to generate. Must be 2 <=  n <= 2^20 
			  output P list of all primes <= n.
			      assert 2<= n <= 2^20
			      //initialize these as set to one
			      (b_2, b_3...b_n) <- (1, 1...1)    
			       i <- 2
			       while i^2 <= n {
			             //mark all multiples of i as composite with 0
			             for j in 2i, 3i, 4i...[n/i] {
			                b_j <- 0
			              od
			              loop{
			              	i <- i+ 1
			              	until b_i = 1
			              od
			              //all primes flagged with 1
			              P <- []
			              for k in 2, 3...n {
			                 if b_k = 1 then {
			                    P <- P || k                  
			                 }
			              }
			              return P
			             } }
			  ```
	- 10.3 COmputations Modulo a Prime
		- Ok so computing modulo a prime is very important!!!
		- Notation of (*a* mod *b*) is used to denote modulo operations or (mod *p*)
		- Any interger taken modulo *p* is always in the range 0..p-1 even if original interger is negative
		- 10.3.1 Addition and subtraction
			- Sum  cannot exceed *2p - 1* so you have to subtract *p* at most once to get back in proper range
		- 10.3.2 Multiplication
			- First compute *ab* as interger and then take result modulo *p*.
			- *ab* can be as large as (p-1)^2 = p^2 - *2p* + 1
		- 10.3.3
			- Groups and Finite Fields
				- numbers modulo a prime *p* are a *finite field* and refreed as "mod *p*"
					- can add and subtract any multiple of *p* without changing result
					- all results in range of 0,1... p-1
					- algebraic rules of ints still apply
					- use the notation *Z_p* to refer to the finite field modulo *p*
					- #+BEGIN_QUOTE
					  defn: a group- set of numbers toghether with an operations such as addition or multiplication.
					  #+END_QUOTE
					- #+BEGIN_QUOTE
					  multiplictive group modulo *p* is numbers 1..p-1 together with multiplication modulo *p* to form a group
					  #+END_QUOTE
			- 10.3.5 extended euclid algorithm
				- need to compute division modulo *p*. we can  do this via the extended euclidian algorithm. while computing *greatest common divisor(a, b)* we can also find two integers u and v such that *gcd(a, b) = ua + vb*. This allows us to compute *a/b (mod p)*.
				- ``` 
				  Extended GCD
				  input  a: positive int, b: positive int
				  output k:the greatest common divisor of a and b (u, v) ints such that ua + vb = k
				  	assert a>=0 ^ b >= 0
				      (c, d) <- (a, b)
				      (u_c, v_c, u_d, v_d) <- (1,0,0,1)
				      while c != 0 {
				      //invariant (u_c*a + v_c*b = c) ^ (u_d*a + v_d*b = d)
				      	q <- [d/c]
				          (c, d) <- (d - q*c, c)
				          (u_c,v_c,u_d,v_d) <- (u_d - qu_c, v_d - qv_c, u_c, v_c)
				       od
				       return d,(u_d, v_d)
				      }
				  ```
			- if we know the *ExtendedGCD(b,p)* of *b* and *p* is 1, because p is prime and provides two numbers u and v such that *ub* + *vp* = *gcd(b, p)* = 1 and we can thus have inverse modulo a prime *p*, which means we can compute division modulo *p*,
			- togerther with addition and subtraction, multiplication, this means we have a a finite field modulo *p*
		- 10.3.6 Working modulo 2
			- addition modulo 2 is exactly the exlcusive-or (XOR)
		- 10.4 Large primes
-
-
-
- # Homework
- chp 9:
	- Entropy is...a property of matter,
	- It's a measure of randomness or how we can't predict/are uncertain what the next bit in a sequence
	- average number of bits you would need to specify the value if you could use an ideal compression algorithm
	- #+BEGIN_QUOTE
	  Why does a combination (eg. by XOR, or by hash-concatenation as on p145, Reseed) of two or more independent input-streams *{S_1..S_n}* of entropy has at least *H(X)>= max{S_i}*entropy; that is, why a combination of entropy streams is always at least as entropic as the most entropic stream.
	  #+END_QUOTE
	- Because of diffusion within any sequence of bits??
-
-
- # Lecture: Asymetric cryptography
- Chapter 11 + 12
- Diffie hellman: we need a rigirous defn of group is:
- Can Add(), Multiply(), Inverse()
- Cyclic groups give us elements we can do cryptography with, with maximum bit length!
- Every group has an *Identity* and *Closure*
- Prime groups (Intergers modulo a prime too) are upper bounded by bits used to describe them
- Any multiplicative group of modulo prime
- Diffie hellma: sends a group element to power of X, has a known public key to be authenticated by certificate authority.
	- This can then be used to share a secret by exponatie with someone else's private key
	- Discrete log problems come up often in cryptography!
- When we have modulo prime group, we always have p - 1
- Group element to power of order, every element of that will be Identity of that group which is 1, Fermats little theorem
- polynomials also can work as group, with tuple of the coefficents!
	- coefficents are elements of the group
- Man in the middle attacks on diffie hellman?
	- If we dont have way to authenticate an individual, they have to trust certificate authority
	- can claim to be an individual and pretend to be parties exchanging keys
	- certificate authority signs public keys
	- if advesory chooses the generator, then they can narrow the field of possible values for shared secrets, oh no!
	- insecure implementation of generator might not check if group generated is small sub-group (where only value could be the *identity* of group)
- Safe primes
	- Sofie germain primes!
	- No elements should have small order!
	- if a bit size of n, we want at least order of n - 1
	- scaling up by using specific generators to repersent the elements of group
	- Legendre symbol! look it up i guess?
- section 11.8 is good review of this chapter
- chapter 12:
	- TODO I had to go before we covered this in lecture whoops
	- RSA:
		- RSA GTFO paper is worth a read!
		-
		-