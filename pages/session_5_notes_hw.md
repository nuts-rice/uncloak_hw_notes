- # Book notes:
	- Implementation Issues:
		- Remeber weakest link property?
	- 8.1 Creating correct programs
		- #+BEGIN_NOTE
		  A "correct" program is one that behaves exactly according to its specifications
		  #+END_NOTE
		- 8.1.1 Specifications:
			- Many software projects have a document called the functional specification.
			- bUT THEY USUALLY DONT LOL. without clear specs theres no hope for getting a correct program
			- Three stages in specification process:
				- Requirements:
					- Informal descriptiopn of what the program is supposed to acheive. "*What* can i do with this program" rather than "how exactly do i do something"
				- Functional specification:
					- detailed and exhaustive defin of the behavior of the program. has to be things we measure outside of the program.
					- bassically stuff we can test where the program passes or not
				- Implementation design:
					- how the program works internally. Contains all the things we can't test from the outside
			- 8.1.2 Test and fix
				- If we find a bug, implement a test that detects the bug. Check that bug is detected! then fix that bug! then check if test no longer finds the bug
				- Whenever we find a bug, think about what caused it. Any other places where it might occur?
				- keep track of every bug that we find!
			- 8.2 Creating Secure Software
				- Diffence between *correct* software and *secure* software: Correct has a specified functionality, while secure has *lack* of functionality. No matter what attacker does, they can't do what isn't privlaged to them.
				- #+BEGIN_QUOTE
				  Standard implementation tecniques are entirely inadequate to create secure code
				  #+END_QUOTE
			- 8.3 Keeping secrets
				- For a secure channel we have two types of secrets: Keys and data.
				- Both are transient: we dont store them for a long time.
				- Data: only stored while we process each message.
				- Keys: Only storedd for the duration of the secure channel.
				- These are stored in machines memory, however these aren't always secure
					- 8.3.1 Wiping State
						- Wipe any information as soon as we dont need it.
						- If we write a library that others will use, we have to depend on main() to inform of us when state is no longer needed
						- #+BEGIN_EXAMPLE
						  When communication channel is closed, the crypto library should be informed so it can wipe the secure channel session state  
						  #+END_EXAMPLE
						- in OOP things are a bit easier...theres deconstructor functions or operators like ~ anmd the the destructor can wipe the state. C++ ensures that all stack allocated objects are properly destroyed whern the stack is unwound during exception handling, but the program has to wnsure that all heap allocated objects are destroyed
						- So check the code that compiler produces and make sure secrets are actually being wiped
						- All data is loaded into CPU register, but during context switch (when operating system switches from running one program to next program), the values in the registers of the CPU are stored in memory where their values might linger for a bit.
					- 8.3.2 Swap File
						- Most operating systems use a virtual memory system to increase the number of programs that can run in parrallel. When program is running not all of its data is kept in memory, some of it is stored in a swap file.
						- When the program tries to access data not in memory, the program is interrupted. The virtual memory system reads required data from the swap file into a piece of memory and program is allowed to continue
						- When the virtual memory managment system decides it needs more free memory, it will take an arbitary piece of memory from the program and wite it to the swap file
						- THIS COULD INCLUDE TO THE MEMORY WHERE KEYS ARE STORED!!!
						- This data would remain on disk potentionally!
						- How do we stop it?
						- There are system calls that we can use to inform the virtual memory system to not be swapped out!
						- Secure swap system exists as well where swapped out data is cryptographically proteccted
					- 8.3.3 Caches
						- COmputers have hiearchies of memories.
						- Main memory which is relatively slow
						- then theres caches, smaller but faster memory
						- Cache keeps copy of most recently used data from main memory, and is the first one to be checked when accdessing data.
						- Caches keep copies, including copies of secret data!
						- Wiping data might not wipe properly, modifications might only be written to the cache and not the main memory
					- 8.3.4 Data reterntion by memory
-
- # Extra reading:
- A look at session and channel based programming alongside lamda claculus used to define it [[https://www.di.fc.ul.pt/~vv/papers/honda.vasconcelos.kubo_language-primitives.pdf]]
- Documentation on annotations used to flag unsafe state transitions in Rust [[https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent]]
- Lecture notes on type state programming in Rust [[https://stanford-cs242.github.io/f19/lectures/08-2-typestate]]
- Session Types for Rust [[https://munksgaard.me/papers/laumann-munksgaard-larsen.pdf]]
- Lecture notes on session types for Rust [[https://stanford-cs242.github.io/f19/lectures/09-1-session-types]]
- Session types implemented in Rust [[https://github.com/Munksgaard/session-types/blob/master/src/lib.rs]]
	- Session typed channels: always carry a protocol which dictates how communication is to take place
		- #+BEGIN_EXAMPLE
		  Imagine that two threads, 'A' and 'B' want to communicate in following pattern:
		   1. 'A' sends interger to 'B'
		  2. 'B' sends a boolean to 'A' depending on the interger recieved
		  #+END_EXAMPLE
	- Done by a single channel. From A's point of view it has the type *int ! (bool ? eps)*  where *t ! r* is the protocol "send something of type *t* then proceed with the protocol *r*". The protocol *t ? r* is "recieve something of type *t*" then proceed with protocol *r* and "eps" is a special marker indicating end of communication session.
	-
-
-
- # Lecture