# macro's
## Wat voor macro's zijn er allemaal?
Rust kent vele soorten macro's:
* Declarative Macros: Door middel van pattern matching genereert meer code
* Custom Derive Macros: Voegt implementaties toe aan structs of traits.
* Attribute-like Macros: voegt attributen toe aan structs.
* Function-like Macros: Look like function calls but operate on the code.


## _Macros_, _Metaprogramming_ en _Abstract Syntax Tree (AST)_, wat is het allemaal?
In het kort _metaprogramming_ is code dat code schrijft.
Macro's is wat Rust aanbiedt om dit mogelijk te maken.
Fun fact macro's komen van LISP vandaan uit 1963. 🤯


### Abstract Syntax Tree (AST)
Een AST is een vertegenwoordiging van je code voor de compiler. 
[Wat dit precies inhoud gaat dit artikel verder op in.](https://dev.to/balapriya/abstract-syntax-tree-ast-explained-in-plain-english-1h38)