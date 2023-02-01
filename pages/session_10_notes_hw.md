- # book notes:
- Introduction to mathematical cryptography:
- 1.2: Divisibility and greatest common divisors:
	- Set of intergers we use is the symbol *Z*
	- #+BEGIN_QUOTE
	  Set of intergers with their addition and multiplication rules are an  *ring*
	  #+END_QUOTE
	- #+BEGIN_QUOTE
	  Defn: Let *a* and *b* be intergers with b!=0. we say that *b* divides *a* or a is divisible by b if there is such an interger *c* such that *a = bc*
	  #+END_QUOTE
		- [#C] #+BEGIN_QUOTE
		  Proposition: Let *a, b, c in Z* be intergers.
		  (a) if a/b and b/c then a/c   
		  (b) if a/b and b/a then a = +b
		  (c) if a/b and a/c, then a/(b+c) and a/(b-c)
		  #+END_QUOTE
		- #+BEGIN_QUOTE
		  Defn: A common divisor of two intergers *a* and *b* is a positive interger *d* that divides both of them. A greatest common divisior of *a* and *b* is the largest positive *d* such that *d/a* and *d/b*. Denoted as *gcd(a,b)*
		  #+END_QUOTE
		- #+BEGIN_QUOTE
		  Defn: effecient algorithm for computing greatest common divisiors is *division with remainder*. dividing *a* and *b* will get a quotient *q* and remainder *r* where remainder is smaller than *b*
		  #+END_QUOTE
		- #+BEGIN_QUOTE
		  Defn: Division algorithm: let *a* and *b* be positive intergers. Then *a* divided by *b* has quotient *q* and remainder *r* means that 
		  *a = b * q + r* with 0<=r<b
		  #+END_QUOTE
		- #+BEGIN_QUOTE
		  Defn: Eucledian algorithm:repeated divison with remainder
		  #+END_QUOTE
		- ![Screenshot from 2023-01-31 16-19-51.png](../assets/Screenshot_from_2023-01-31_16-19-51_1675200142621_0.png)
		- #+BEGIN_QUOTE
		  Proof: The *r_i* values are strictly decreasing and bound to zero when the algorithm terminates. THis is finite steps. Each iteration of step 3 we have equation of form 
		  *r_i-1 = r_i * q_i+r_i+1* 
		  #+END_QUOTE
		- this equation implies that any common divisor of *r_i-1* and *r_i* is also a divisor of *r_i+1* so...
		- #+BEGIN_QUOTE
		  gcd*(r_t-1, r_t)* = gcd(*r_t*q_t, r_t*)=*r_t
		  #+END_QUOTE
		- this is equal to gcd*(r_0, r_1)*  i.e to gcd(*a,b*)
		- nonzero remainder in euclidean algorithm is equal to greatest common divisior of *a* and *b*
		- since *r_i* values are strictly decreasing, the algorithm terminates in most *b* steps.
		- FOr every two iterations of step 4 the value of *r_i* is at least cut in half
		- #+BEGIN_QUOTE
		  In other words: 
		  *r_i+2* < 1/2* *r_i* for all *i* = 0,1,2...
		  #+END_QUOTE
	- #+BEGIN_QUOTE
	  Extended eucledian algorithm:
	  Let *a* and *b* be positive intergers. Then the equation
	  *au* + *bv* = gcd(*a, b*)
	  always have solutions for intergers *u* and *v*
	  
	  If (*u_0, v_0*) is any one solution, then every solution has form :
	  *u* = *u+0*+ *b * k* / gcd(*a, b*) and *v* = *v_0* - *a* * *k* / gcd(*a,b*)
	  for some *k* in *Z*
	  #+END_QUOTE
		- at each stage of EEA we find that *r_i* is the sum of an interger multiple of *a* and interger multiple of *b*. Eventually we get to *r_t* = *a* * *u* + *b* * *v* for some intergers *u* and *v*
		- Important case of EEA is when gcd of *a* and *b* is 1. In this case we call *a* and *b* *relatively prime*
		- EEA tabular:
		- |||*q_1*|*q_2*|||*q_t-1*|
		  |--|--|--|--|--|--|--|
		  |0|1|*P_1*|*P_2*|...||*P_t-1*|
		  |1|0|*Q_1*|*Q_2*|...||*Q_t-1*|
	- 1.3 Modular arthimatic
		- #+BEGIN_QUOTE
		  Let *m* >= 1 be an interger. We say that intergers *a* and *b* are *congurent modulo m* if their difference *a-b* is divisible by *m*
		  *a≡b* (mod *m*)
		  #+END_QUOTE
		- the number *m* is called the modulus. Numbers satisfying *a*≡0(mod *m*) are numbers divisible by *m*, the multiples of *m*
		- congrunces  behave much like equalities
			- #+BEGIN_QUOTE
			  Let *m* >= 1 be an interger
			  (a) If *a_1* ≡ *a_2* (mod *m*) and *b_1*≡*b_2*(mod *m*) then
			  *a_1* ± *b_1* ≡ *a_2* ± *b_2* (mod *m*) and *a_1* * *b_1* ≡ *a_2* * *b_2* (mod *m*)
			  (b) Let *a* be an interger. Then
			  *a* * b ≡ 1 (mod *m*) for some interger *b* if and only if gcd(*a*, *m*) = 1.
			  If such an interger *b* then we say that *b* is the *multiplicative inverse* of modulo *m*.
			  (any two inverses are congurent modulo *m*)
			  #+END_QUOTE
				- if gcd(*a*,*m*) = 1 then there exists an inverse *b* of *a* modulo *m*. The fraction *b^-1* = 1/*b* then is a meaningful interpretation in the world of integers modulo *m*
				- #+BEGIN_NOTE
				  When *m* is large, it is challenge to compute *a^-1* modulo *m*. They exist by the EEA. In order to actually compute the *u*  and *v* that appear in *au + mv* = gcd(*a, b*) we can apply Eucledian algorithm directly or we can use box method or other algorithm. Since the euclidian algorithm takes only 2 log_2(*b*) + 3 iterations to compute gcd(*a, b*), it takes only a small multiple of log_2(*m*) steps to compute *a^-1* modulo *m*
				  #+END_NOTE
			- If *a* divided by *m* has quotient *q* and remainder *r* it can be written as:
				- *a* = *m* * *q* + *r* with 0 <= *r* <= *m*
					- *a* ≡ *r* (mod *m*) for some interger *r* between 0 and *m* - 1, so if we work with intergers modulo *m*, its enough to use intergers 0 <= *r* <= *m*
				- #+BEGIN_QUOTE
				  Defn: *Z* / *m**Z* = {0,1,2....*m*-1}
				  and call *Z*/*m**Z* the *ring of intergers modulo* *m*
				  #+END_QUOTE
					- whenever we perform addition or multiplication in *Z*/*m**Z* we divide the result by *m* and take the remainder in order to obtain an element in *Z*/*m**Z*
					-
	-