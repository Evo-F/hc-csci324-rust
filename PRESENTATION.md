**Basic Overview of Characteristics/Paradigms**
- Strongly-typed
- Systems programming language
    - Very familiar to anybody who's used C/C++, minus the object-oriented aspects of C++
- Performance on par with C/C++
- Focus on performant memory safety (meaning the language is safe without sacrificing performance) and stability 

**Brief History of Rust**
- Originated in 2011 (possibly the newest language being presented? needs confirmation) from Mozilla
    - Yes, *that* Mozilla
- Recently approved by Google for use in the Android kernel, now being pushed for acceptance in both Windows **and** Linux kernels 
    - This is owed to Rust's focus on memory safety without compromising on performance. 
- Consistently voted the most loved language by StackOverflow users, despite a majority of users not indicating Rust as their primary language
    - The people who use it **really love it**

**Technical Features (Unique to Rust)** 
- Ownership and Scope-Changing
    - Rust's memory stability and efficiency is the result of several rules enforced at compile-time. These rules are detailed (to the best of our ability) in a later section.
    - The developer is not responsible for manually managing memory (C/C++), nor is there a garbage collector which impacts performance (Java). 
- Explicit Error Handling
    - Rust has no ` null ` type (well it almost does, but it's buried in the standard library and doesn't work the way one might expect), so any code that COULD return a null value instead returns an ` Option<T> `. 
    - That ` Option<T> ` must then be resolved by the developer to access the underlying data, while also explicitly handling the ` None ` case. 
    - The user should never see a "weird memory error" or null pointer exception, even if they do everything wrong and completely misuse the program.
- Ease of Compilation/Production
    - All of Rust's compilation options are contained within a single ` Cargo.toml ` file. 
    - Unlike more unwieldy build tools like Maven (Java) or CMake (C/C++), Cargo is bordering on human-readable and is highly extensible with configuration options.
- Mutable/Immutable References and Usage
    - Reference rules! Coming up in a later section!
- Stack vs. Heap - Developer's Choice!
    - Any memory that *would* be allocated on the stack can be forced to allocate on the heap with ` Box() `. 
    - Most data types, including developer-made structs, are allocated on the stack by default for performance. 
- Traits
    - Rust's equivalent of Java interfaces. If a struct has a given trait, the compiler can guarantee it has certain behavior associated with it.
    - It is up to the developer to actually write that behavior.
    - There are several *very important* default traits which the developer must implement on their own structs by hand if they wish to use them. 
    - Example: ` Copy ` - Tells the compiler to COPY a piece of data when it is passed as a parameter. Without this trait, the data is MOVED instead, changing scope.

**Technical Features (seen in other langages)**
- Robust Pattern-Matching
- "Quick Return" Statements
    - Functions can return data without the ` return ` keyword.
- Functions are top-level objects

**The Rules of Rust**
- The user should never see an error the developer did not write themselves.
- Immutable and mutable references to the same memory may never coexist. Only one mutable reference to a piece of memory may exist at any one time. 
- When data goes out of scope, it is dropped unconditionally. 
- Data goes out of scope after its last use before a function/loop closure. 
    - Scopes can also be created arbitrarily just by using braces {}.
- When data cannot be copied, it is moved. 
    - This is **wholly different** from the approaches to parameter passing we have looked at in class. 
    - It is most similar to Pass-by-Reference, but the called function now *owns that piece of data* - the calling code no longer owns it.
    - This means the data has **changed scope**, so when the function ends, it's gone.
