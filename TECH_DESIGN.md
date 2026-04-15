# Technical Design Document — Centurion: 100 Steps

## 1. Stack Tecnologico

- **Linguaggio**: Rust (Edizione 2021)
- **Engine**: Bevy (Versione 0.13+)
- **Fisica**: Nessuna (Determinismo puro a griglia)
- **Rendering**: Bevy Sprites (Forme geometriche semplici: cerchi, quadrati)
- **UI**: Bevy UI (Design minimale, bianco e nero)

---

## 2. Stati del Gioco (`GameState`)

- `Loading`: Caricamento configurazioni piani e nemici.
- `Room`: Fase di pianificazione e movimento.
- `CombatEvent`: Animazione rapida di risoluzione scontro.
- `Rest`: Fase tra i piani (scelta bonus/altare).
- `Dead`: Schermata finale di riepilogo passi effettuati.

---

## 3. Architettura ECS & Moduli

### Componenti Chiave
- `Centurion`: Tag per il giocatore. Contiene `CurrentSteps` e `Force`.
- `Enemy`: Contiene `Force` e `Behavior` (es. statico, inseguitore).
- `TimeFragment`: Ricarica il contatore dei passi.

### Moduli (Plugins)
- `tactics`: Logica di calcolo del movimento e dei costi in passi.
- `resolver`: Calcolo deterministico degli scontri (senza PRNG).
- `map_gen`: Creazione delle stanze chiuse 8x8 con seeding deterministico.

---

## 4. Meccaniche Core (Dettagli)

### Combattimento Deterministico
Quando il giocatore tenta di muoversi su una cella occupata da un nemico:
1. `Player.Force - Enemy.Force = NewPlayerForce`.
2. Se `NewPlayerForce <= 0`, il giocatore muore.
3. Se `NewPlayerForce > 0`, l'ente nemico viene rimosso e il giocatore occupa la sua posizione.
4. **Strategia**: Il giocatore deve cercare oggetti che aumentano la `Force` prima di affrontare nemici necessari per raggiungere l'uscita.

### La Risorsa Passi
Ogni mossa riduce `CurrentSteps`. Se `CurrentSteps == 0`, il giocatore non può più muoversi. Nessun nemico si muoverà finché il giocatore non ha effettuato il suo turno.

---

*Ultima revisione: 2026-04-14*
