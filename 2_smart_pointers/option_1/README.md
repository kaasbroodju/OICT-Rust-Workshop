# Option<_T_>

Soms kan een waarde uit niets bestaan.
Deze waarde wordt vaak null genoemd.

Rust heeft geen null, maar de option type (stiekem een enum).
Als het de waarde _None_ heeft wijst de option type naar null/nullptr
Als het wel een waarde bevat de option een pointer naar de waarde.*

_*Vanwege Rust ownership regels kan Rust optimaliseren dat het geen pointers zijn, maar direct verwijst naar jouw waarde._