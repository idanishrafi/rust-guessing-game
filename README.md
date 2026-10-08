# 🦀 Guessing Game

Built this quick project while revisiting *The Rust Programming Language* book to brush up on standard I/O, error handling, and crate management.

---

## ⚡ The Prelude

You find yourself standing at the precipice of probability. 

The machine has chosen a secret integer within the closed interval $[1, 100]$. It does not care about your intuition, your luck, or your hubris. You are granted exactly **5 attempts** to match its cold, deterministic choice. 

Step into the terminal—if you dare.

---

## ⚔️ Rules of Engagement

* **5 Strikes Only**: You have 5 chances to decipher the answer before complete system failure.
* **Tactical Feedback**: On attempts 1 through 4, the machine will analyze your input and signal `Try Higher` or `Try Lower`.
* **No Mercy on Attempt 5**: Should you reach your final attempt, **all hints are suppressed**. You will receive no guidance, no safety net, and no second chances. You either strike true, or face the cold reality of the reveal.

---

## 🚀 Running the Game

Ensure you have Rust installed, then clone the repository and run:

```bash
cargo run
