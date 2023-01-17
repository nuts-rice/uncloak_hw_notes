- # Book notes:
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