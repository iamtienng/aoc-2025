# Day 12: Christmas Tree Farm - Part 1 Solution

## Problem Formulation

Given:

- A set of shapes **S** = {s₀, s₁, ..., sₙ}
- A set of regions **R** = {r₁, r₂, ..., rₘ}
- For each region rⱼ with dimensions Wⱼ × Hⱼ, a requirement vector **cⱼ** = (c₀, c₁, ..., cₙ)

Determine: |{rⱼ ∈ R : rⱼ is feasible}|

## Algorithm

### Preprocessing Phase

For each shape sᵢ ∈ S:

- Compute A(sᵢ) = area of shape (number of cells)
- Generate transformation set T(sᵢ) = {all unique rotations and reflections}
- Extract orientation set O(sᵢ) = {(w, h) : bounding box dimensions of each t ∈ T(sᵢ)}

### Feasibility Test

A region rⱼ with dimensions Wⱼ × Hⱼ and requirements **cⱼ** is feasible if:

**Condition 1** (Area Constraint):

```
∑ᵢ (cᵢ · A(sᵢ)) ≤ Wⱼ · Hⱼ
```

**Condition 2** (Geometric Constraint):

```
∀i : cᵢ > 0 ⟹ ∃(w, h) ∈ O(sᵢ) : w ≤ Wⱼ ∧ h ≤ Hⱼ
```

**Condition 3** (Validity):

```
∀i : cᵢ > 0 ⟹ sᵢ is defined ∧ A(sᵢ) > 0
```

### Output

Count regions satisfying all three conditions.

## Complexity Analysis

- **Time**: O(m · n) where m = |R|, n = |S|
- **Space**: O(n · k) where k = |T(sᵢ)| ≤ 8

## Correctness

The algorithm computes necessary (but not sufficient) conditions for feasibility. The conjunction of these conditions provides a lower bound on solvability for the given input instances.
