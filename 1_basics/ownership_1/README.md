# Ownership
Rust kent een vrij nieuw concept, namelijk _'ownership'_.
Dit concept is het best te omschrijven in 3 regels.
1. Elke waarde kent een eigenaar.
let x = 4;
Eigenaar: x
waarde: 4
2. Elke waarde heeft exact 1 eigenaar
let x = 4;
let y = x;
let z = x; // fout error (meerdere eigenaren)
3. Wanneer de eigenaar uit de scope valt wordt de waarde _'gedropped'_

![8q8rdy](https://github.com/kaasbroodju/OICT-Rust-Workshop/assets/35763691/d38e7a59-930b-456c-8ef8-e40d87a89e08)


## Wat wordt er bedoelt met _'scope'_ en _'gedropped'_?
### Scope
Een scope houdt in hoe lang een variable leeft.
Als een variable het einde bereikt van een curly bracket, wordt die gedropped.

```rust
// scope
{
    let x = 1;
}

// Method scope
fn test() {

    let a = 1;
    // inline-scope
    let b = {
        let c = 1;
        c
    };
    
    // if block scope
    let d = if true {
        let e = "hello";
        e
    } else {
        "world"
    };
    
    a
}
```


### 'dropped'
Een variable _'droppen'_ houdt in dat je het geheugen die de variable innam weer vrijgeeft aan de OS.
Dit gebeurt vaak pas een het einde van scope of handmatig. 
