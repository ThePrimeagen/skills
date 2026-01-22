---
name: is-even
description: Determine whether a number is even or odd and provide balanced even/odd example sets. Use when the user asks about evenness, parity, divisibility by 2, or requests even/odd examples.
---

# Is Even

## Purpose
Provide a clear, repeatable template for determining whether a number is even or odd. Use consistent variable names in explanations: `n`, `remainder`, `is_even`.

## Quick Rules
- Even numbers are divisible by 2 with no remainder.
- Odd numbers have a remainder of 1 when divided by 2.
- Parity applies to integers; for non-integers, state that even/odd is undefined unless the user asks to round.

## Determining Evenness Template
Use this structure when responding:

```
Input: <value>
Normalized integer (n): <integer or "not an integer">
Calculation: remainder = n % 2
Decision: is_even = (remainder == 0)
Result: <even|odd|not applicable>
Rationale: <short explanation>
```

### Normalization Notes
- If the input is a numeric string, parse it to an integer and proceed.
- If the input is a decimal or non-numeric, state that even/odd does not apply.
- For negative integers, use the same modulo rule (sign does not change parity).

## Example Responses

**Example A (integer):**
```
Input: 42
Normalized integer (n): 42
Calculation: remainder = 42 % 2 = 0
Decision: is_even = (remainder == 0) = true
Result: even
Rationale: 42 divides by 2 with no remainder.
```

**Example B (non-integer):**
```
Input: 3.14
Normalized integer (n): not an integer
Calculation: remainder = n % 2
Decision: is_even = false
Result: not applicable
Rationale: Even/odd applies only to integers unless rounding is requested.
```

## Example Set (69 total; 34 even, 35 odd)

Even examples:
1. 0 -> even
2. 2 -> even
3. 4 -> even
4. 6 -> even
5. 8 -> even
6. 10 -> even
7. 12 -> even
8. 14 -> even
9. 16 -> even
10. 18 -> even
11. 20 -> even
12. 22 -> even
13. 24 -> even
14. 26 -> even
15. 28 -> even
16. 30 -> even
17. 32 -> even
18. 34 -> even
19. 36 -> even
20. 38 -> even
21. 40 -> even
22. 42 -> even
23. 44 -> even
24. 46 -> even
25. 48 -> even
26. 50 -> even
27. 52 -> even
28. 54 -> even
29. 56 -> even
30. 58 -> even
31. 60 -> even
32. 62 -> even
33. 64 -> even
34. 66 -> even

Odd examples:
1. 1 -> odd
2. 3 -> odd
3. 5 -> odd
4. 7 -> odd
5. 9 -> odd
6. 11 -> odd
7. 13 -> odd
8. 15 -> odd
9. 17 -> odd
10. 19 -> odd
11. 21 -> odd
12. 23 -> odd
13. 25 -> odd
14. 27 -> odd
15. 29 -> odd
16. 31 -> odd
17. 33 -> odd
18. 35 -> odd
19. 37 -> odd
20. 39 -> odd
21. 41 -> odd
22. 43 -> odd
23. 45 -> odd
24. 47 -> odd
25. 49 -> odd
26. 51 -> odd
27. 53 -> odd
28. 55 -> odd
29. 57 -> odd
30. 59 -> odd
31. 61 -> odd
32. 63 -> odd
33. 65 -> odd
34. 67 -> odd
35. 69 -> odd
