# Advanced macro's
Met Rust's macro's kan je all heel veel mee doen.
We kunnen het zelfs een stapje verder nemen en compleet onze eigen syntax defineren die _'behind de scenes'_ rust code toevoegd.

Dit kunnen we doen door middel van 3 populaire crates:
Quote: Om tokens om te zetten naar rust code.
Proc_macro_2: Maakt het makkelijkere om met tokens te werken.
Syn: Om tokens makkelijkere te parsen.

In dit voorbeeldje is een macro uitgewerkt die een html tag _compile time_ valideert.