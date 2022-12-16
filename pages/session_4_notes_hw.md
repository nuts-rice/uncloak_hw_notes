- AEADS, MAC tags, CHACHA20 poly, TLS
- No session next week!
- Authenticated Encryption
	- Key terms: AEAD, MAC tag, AES-GCM, ChaCha20 Poly 1305
	- PREVENTABLE VULNERABILITIES LED TO aead
	- More steps needed to verify client: less ddos exposure!
	- Answer is in Authenticated encryption:
		- AES GCM mode and ChaCha20-poly1305
		- needed to follow standards, multiple algos: cryptographic agility
		- Theres specific chips for computing AES
		- #+BEGIN_NOTE
		  rust libs for hardware crpto instruction sets:
		  config feature branches, feauture gated for specific instruction sets/operating sets (malicious behavior: xlr-tevm, theashold signatures)
		  #+END_NOTE
		- but those without them like phones: google proposed standard for Spdy, using chacha20
	- Spot check: AEADs operate on stream cipher that encrypts then takes MAC,
- HW:
	- Different modes:
	- Authenticate, then encrypt vs encrpt then auth
	- more AEADs choose encrpt then auth
	- objectives for next weeks:
		- Constant time complexity,control flow of secret data, dont leak memory, implentation and primatives, memory model of rust!
		- check resources!
		- Rust would use subtle library, 1
		- rust 1.6: blackbox primative!!
		- seperte securitty and abstractrion layer
		- zeroeyes, secrecy