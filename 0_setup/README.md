# Setup
## Download & installatie
Ga naar: https://rustup.rs/ en volg de instructies.

### Verifiëren
Om te checken of alles correct is geïnstalleerd run de volgende command: ```rustup update```

## Creëer een project
Nu dat we alle Rust benodigdheden hebben geïnstalleerd, kunnen we nu onze eerste project starten.
Dit doen we via _Cargo_.

### _Maar wat is Cargo?_
Cargo is de package manager van Rust.
Je kunt het een beetje vergelijken met NPM, PIP, Maven, maar het doet ook veel meer.
Cargo compileert je project, voert unit tests uit, kan je code benchmarken en nog veel meer.

Om een project te initialeren runnen we de volgende commando: ```cargo init hello_world```

Voor deze workshop hoef je maar een paar cargo commando's nodig.

Om je project te draaien:
```cargo run```

Om je project te testen:
```cargo test```

Om je project te benchmarken:
```cargo bench```
