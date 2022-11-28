- Notes:
	- Chapter 2:
		-
- Homework:
	- chapter 1:
		- Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.
			- #+BEGIN_EXAMPLE
			  CVE-2019-11510-people connecting to a "secure" enterprise VPN only for there to specific requests that attackers can send to users on that VPN and perform RCE
			  #+END_EXAMPLE
	- chapter 2:
		- Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.
			- Handshake lemma: counting number of handshakes in a room with n + 1 people, handshkes once per person. So (n(n - 1) / 2) = 435 keys to exchange.
		- Q4. Suppose Bob receives a message signed using a digital signature scheme with Alice's secret signing key. Does it prove that Alice saw the message and chose to sign.
			- could be adversary that recorded message and secret signing key? might also need numbering scheme alongside signature to fully prove alice signed
			- pg 29: alice trusts the computer to compute the signature,  viruses could infect computer and forge the signatures so nope basically
		- Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?
			- A "good" encryption algo should be able to easily withstand chosen-plaintexts. Attacker has more freedom in a chosen-ciphnertext attack as you have both plaintext and corresponding plaintext.
				- However theres *other* factors to the security of an algorithm:
					- An attacker could recover info about an message which is *broken* diffusion while still having *successful* confusion. If one bit doesnt change when it should to be secure, thats *bad*
					-