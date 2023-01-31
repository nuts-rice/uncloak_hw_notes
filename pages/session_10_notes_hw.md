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
		-