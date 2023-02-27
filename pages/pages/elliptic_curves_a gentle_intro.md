- [url](https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)
- #+BEGIN_IMPORTANT
  Described by the equation:
  #+END_IMPORTANT 
  #+BEGIN_EXPORT latex
  y^{2} = x^{3} + ax + b
  #+END_EXPORT
  where 
  #+BEGIN_EXPORT latex
  4a^{3} + 27b^{2} \neq 0
  #+END_EXPORT
- #+BEGIN_NOTE
  Defn: The *Weierstrass normal form* is the equation above
  #+END_NOTE
- They are symetrric along the x-axis
- #+BEGIN_IMPORTANT
  we also need a *point at infinity* to be part of this curve which denoted as 0
  #+END_IMPORTANT
- Now all that can be written as 
  ![image.png](../assets/image_1677427814226_0.png)
- Groups:
	- we have binary operation of addition or + which is defined by:
		- *closure*: if *a* and *b* are members of *G* then *a* + *b* is member of *G*
		- *associativity*: (a + b) + c = a + (b + c)
		- there's an *identity element* 0 such that a + 0 = 0 + a = a
		- every element has *iverse*, for every *a* there exists *b* such that *a* + *b* = 0
		- abelian requirement:
		- *communativity*: a + b = b + a
		- from these can be derived
			- the *identity element is unique*
			- *inverses are unique*
- Group law for elliptic curves:
	- elemets of group are the points of an elliptic curve
	- identity element is point at infinity 0
	- invers of a point *P* is the one symmetric about the *x*-axis
	- addition rule: given three alligned, non-zero points *P*, *Q*, *R*, their sum is P + Q + R = 0
		- don't need an order and whern they are aligned....
		  P + (Q + R) = Q + (P + R) = R + (P + Q) = 0 which means ...
		  abelian group with associastivity and communitative
- Geometric addition:
	- So if P + Q + R = 0...we can have P + Q = -R (the inverse)
	- ![image.png](../assets/image_1677455512901_0.png)
	- But because of this we also need to ask..
		- What if P = 0 or Q = 0...?  P + 0 = P and 0 + Q = Q for all P and Q
		- what if P = -Q? That's vertical and doesn't pass through a third point
		- What if P = Q? There's infintely many lines... But consider Q' != P.
		  If Q' approaches P then it get closer to it
			- As Q' goes towards P, the line passing through P and Q' becomes tangent to curve.
			- ![image.png](../assets/image_1677456610843_0.png)
		-