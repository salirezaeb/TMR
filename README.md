# 🧠 TMR Monte Carlo Simulation 
### Fault Tolerant System Design course assignment under the supervision of Dr. Zarandi at Amirkabir University of Technology
This repository contains a **Monte Carlo simulation** of a **Triple Modular Redundancy (TMR)** system implemented in **Rust**.  
It compares two voting strategies:

- **Classic majority voter** (2-of-3 voting with tie-break)
- **Reliability-aware MAP (Maximum A Posteriori) voter** (probabilistic decision rule using module reliabilities)

The objective is to quantify how a reliability-aware voter improves correctness when module reliabilities are unequal and outputs are multi-valued.

---

## 🧩 System Model

- Three independent modules operate in parallel (TMR)
- The correct system output is fixed and equal to **27**
- Each module produces an output in the range **0–63**

### Probabilistic Output Model (per module)
For module *i* with reliability **Ri**:

- Probability of correct output (27): **Ri**
- Probability of an incorrect output (any value except 27): **(1 − Ri) / 63**

This reflecThis assumption states that, in the event of a module failure, the output is uniformly distributed over the 63 incorrect values in the integer range 0–63, excluding the correct value 27.

### Odd Case Reliabilities
According to the problem specification, the module reliabilities depend on the last digit of the student ID.  
Since the last digit of the student ID is **13**, which is **odd**, the following reliability values are used:

- **R1 = 0.9**
- **R2 = 0.5**
- **R3 = 0.2**

---

## 🗳️ Voting Algorithms

### 1) Classic Majority Voter
Given module outputs `(o1, o2, o3)`:

- If at least two outputs are equal, output that value.
- If all three outputs differ, apply a **random tie-break** and return one of the three values uniformly at random.

This models standard TMR voting behavior (2-out-of-3 agreement), extended to multi-valued outputs where a “no majority” situation can occur.

### 2) Reliability-Aware MAP Voter
The MAP voter selects the candidate value that is most likely to be the true output given:
- the observed outputs, and
- the reliability parameters `{Ri}` and the uniform fault model.

For a candidate value `v`, define the likelihood score:

- If `oi == v`, the i-th module is treated as “correct” under hypothesis `v`, contributing `Ri`.
- If `oi != v`, the i-th module is treated as “incorrect” under hypothesis `v`, contributing `(1 − Ri)/63`.

Thus, for candidate `v`:

Score(v) = product over i=1..3 of:
- `Ri` if `oi == v`
- `(1 − Ri)/63` if `oi != v`

The MAP voter outputs the `v` that maximizes `Score(v)`.  
In the implementation, candidates are restricted to the observed outputs `{o1, o2, o3}`.

---

## 🔁 Monte Carlo Simulation

### Procedure
For each of `N` trials:

1. Generate outputs from the three modules according to the probabilistic model.
2. Compute the output of:
   - Classic voter
   - MAP voter
3. Count a success when the voter output equals **27**.

### Outputs Reported
- `classic_ok`: number of trials where the classic voter returned 27
- `map_ok`: number of trials where the MAP voter returned 27
- `classic_rate = classic_ok / N`
- `map_rate = map_ok / N`

The results of the evaluation are summarized in the following plot:

- [TMR_Comparison.png](./TMR_Comparison.png)


---

## 🎲 Reproducibility and the Role of the Seed

### Why a Seed Is Used
Monte Carlo methods rely on pseudo-random number generators (PRNGs).  
A PRNG produces a deterministic sequence of numbers given an initial state. The **seed** defines that initial state.

Using a fixed seed is important for **scientific and academic reproducibility**:

- Ensures the experiment is **repeatable**
- Allows others (or the grader) to reproduce the exact same results
- Enables fair comparisons when changing parameters (e.g., switching voters or changing N)

### What Happens Without a Fixed Seed
If the PRNG is initialized from system entropy/time, each run uses a different random sequence. Results will vary slightly from run to run, which is normal for Monte Carlo experiments but less suitable for graded reports requiring consistent outputs.

In this project:
- `seed = 7` is simply a chosen constant (any integer is acceptable).
- The key requirement is **consistency**, not the specific numeric value.

---

## 🖼️ Output Artifact

The program generates a colored, high-resolution bar chart:

- **File:** `TMR_Comparison.png`
- **Content:** side-by-side comparison (Classic vs MAP)
- **Annotations:** count and success rate are displayed on the plot, and axes/labels are included.

---

## ▶️ Build and Run

### Requirements
- Rust (edition 2021 or newer)
- Cargo

### Execution
```bash
cargo run --release -- 1000 7
