- # Lecture
	- Zero knowledge:
		- Some function F(x), but function is expensive, we can hide commitment proof and give proof of verifiable
		- *Completeness, Soundness* are in an argument system
		- honest prover would always get something a verifier would believe
		- Soundness: dishonest prover would have very low probability to be verifier, can use low number of bits of security
		- Zero-knowledge: *simulator* output of possible input
		- example: color blindness:
			- prover sees color, verifer will verify prover can see color
			- two circle with dots of either *green and red* or just *red*
			- repeated circuts of prover verfying = more chance of being honest, more bits of security
			- we could have a DSL or [commitment](logseq://graph/pages?block-id=64120fa0-64d9-4182-a441-7e93eb0c1820) scheme for this specific transcript of challenges and responses
			- cheating prover would have a specific probability
			- simulator comes with reverse order of how the transcript should go.
		- we also can do zk addition, using previous protocol to prove x+y = z
			- do logical circuts of AND with binary sequences of x \cap y
			- this means turing completeness and have arithimetic circuts!
			- one to one function of circuts to compute to boolean logic
			- gkr protocol, fast foueir transforms for snarks and starks
- # Books:
	- What can be shown in zero-knowledge?
		- knowledge of wheter two graphs are isomorphic using a commitied secret random permutation
			- we say that the two graphs are
			  *isomorphic* if there is a relabelling (i.e. a permutation) of the vertices of one graph which produces
			  the second graph. This relabelling φ is called a *graph isomorphism*, which is denoted by
			  φ : G 1 −→ G 2 .
		- we can think NP problems as those problems where there is a witness (or proof) which is proven by all powerful prover but with a polynomial bound verifier that verifies this proof
		- if we allow interaction then....assuming all powerful prover and polynomial bound verifier
		- problems gere are the complexity class of interactive proofs or IP. these are equal to problems in PSPACE, or problems solved using polynomial space.
		- #+BEGIN_NOTE
		  Example: 3-colorability of graph lies in problems that can be verified by zk proofs 
		  #+END_NOTE
			-