**Basic Overview of Characteristics/Paradigms**
- Strongly-typed
- Systems programming language
--- Very familiar to anybody who's used C/C++, minus the object-oriented aspects of C++
- Performance on par with C/C++
- Focus on performant memory safety (meaning the language is safe without sacrificing performance) and stability 

**Brief History of Rust**
- Originated in 2011 (possibly the newest language being presented? needs confirmation) from Mozilla
--- Yes, *that* Mozilla
- Recently approved by Google for use in the Android kernel, now being pushed for acceptance in both Windows **and** Linux kernels 
- Consistently voted the most loved language by StackOverflow users, despite a majority of users not indicating Rust as their primary language
--- The people who use it **really love it**

**Technical Features (Unique to Rust)** 
- Ownership and Scope-Changing
- Explicit Error Handling
- Ease of Compilation/Production
- Mutable/Immutable References and Usage
--- Reference rules! 
- Stack vs. Heap - Developer's Choice!
--- Any memory that *would* be allocated on the stack can be forced to allocate on the heap with ` Box() `. 
--- Most data types, including developer-made structs, are allocated on the stack by default for performance. 

**Technical Features (seen in other langages)**
- Robust Pattern-Matching
- "Quick Return" Statements
--- Functions can return data without the ` return ` keyword.

**The Rules of Rust**
- The user should never see an error the developer did not write themselves.
- When data goes out of scope, it is dropped unconditionally. 
- Immutable and mutable references to the same memory may never coexist. Only one mutable reference to a piece of memory may exist at any one time. 
