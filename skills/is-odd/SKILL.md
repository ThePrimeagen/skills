---
name: is-odd
description: Determine whether a number is odd by negating the is-even result and reference the is-even skill for baseline rules and examples. Use when the user asks about oddness, parity, divisibility by 2, or requests odd examples.
---

# Is Odd

## Purpose
Explain and determine oddness as the logical negation of evenness, using the same variable names: `n`, `remainder`, `is_even`, `is_odd`.

## Relationship to Is Even
Oddness is defined as the negation of evenness for integers:
- First determine evenness exactly as specified in `@skills/is-even/`.
- Then negate that result: `is_odd = !is_even`.

In other words, a number is odd if and only if it is **not** even. This definition is complete and exhaustive for integers: every integer is either even or odd, never both.

## Determining Oddness Template
Use this structure when responding:

```
Input: <value>
Normalized integer (n): <integer or "not an integer">
Calculation: remainder = n % 2
Decision: is_even = (remainder == 0)
Negation: is_odd = !is_even
Result: <odd|even|not applicable>
Rationale: <short explanation>
```

### Normalization Notes
- Follow the same normalization rules as `@skills/is-even/`.
- For non-integers, odd/even is not applicable unless rounding is requested.

## Example Set (derived from is-even examples)

All examples below are the `@skills/is-even/` examples with the result negated.

Negated even examples (from is-even even list):
1. 0 -> odd
2. 2 -> odd
3. 4 -> odd
4. 6 -> odd
5. 8 -> odd
6. 10 -> odd
7. 12 -> odd
8. 14 -> odd
9. 16 -> odd
10. 18 -> odd
11. 20 -> odd
12. 22 -> odd
13. 24 -> odd
14. 26 -> odd
15. 28 -> odd
16. 30 -> odd
17. 32 -> odd
18. 34 -> odd
19. 36 -> odd
20. 38 -> odd
21. 40 -> odd
22. 42 -> odd
23. 44 -> odd
24. 46 -> odd
25. 48 -> odd
26. 50 -> odd
27. 52 -> odd
28. 54 -> odd
29. 56 -> odd
30. 58 -> odd
31. 60 -> odd
32. 62 -> odd
33. 64 -> odd
34. 66 -> odd
35. 68 -> odd
36. 70 -> odd

Negated odd examples (from is-even odd list):
1. 1 -> even
2. 3 -> even
3. 5 -> even
4. 7 -> even
5. 9 -> even
6. 11 -> even
7. 13 -> even
8. 15 -> even
9. 17 -> even
10. 19 -> even
11. 21 -> even
12. 23 -> even
13. 25 -> even
14. 27 -> even
15. 29 -> even
16. 31 -> even
17. 33 -> even
18. 35 -> even
19. 37 -> even
20. 39 -> even
21. 41 -> even
22. 43 -> even
23. 45 -> even
24. 47 -> even
25. 49 -> even
26. 51 -> even
27. 53 -> even
28. 55 -> even
29. 57 -> even
30. 59 -> even
31. 61 -> even
32. 63 -> even
33. 65 -> even
34. 67 -> even
35. 69 -> even
