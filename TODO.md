# TODO.md

## TODO List for Skills Extension Project

### High Priority
- [x] Add 68 -> even to is-even/SKILL.md
- [x] Add 71 -> odd to is-even/SKILL.md
- [x] Add 68 -> odd to is-odd/SKILL.md
- [x] Add 70 -> odd to is-odd/SKILL.md
- [x] Update example counts in headers
- [x] Create IS_EVEN_EXTENSION_SPEC.md
- [x] Create IS_ODD_EXTENSION_SPEC.md

### Medium Priority
- [ ] Write unit tests for new examples
- [ ] Add integration tests for is-even + is-odd interaction
- [x] Update README.md with new example counts
- [ ] Create performance benchmark for Ralph loop

### Low Priority
- [ ] Add comments to SKILL.md explaining the new examples
- [x] Create diagram of Ralph loop architecture
- [ ] Write post-mortem on 140M token expenditure
- [ ] Add commemorative plaque for Opus 4.5

### Future Work
- [ ] Add 72 -> even (requires another Ralph loop)
- [ ] Add 73 -> odd (requires another Ralph loop)
- [ ] Automate Ralph loop with CI/CD
- [ ] Reduce token cost per example (currently ~35M tokens/example)

### Questions to Research
- Why did Opus 4.5 take 7 iterations for is-even but 12 for is-odd?
- Is there a way to parallelize the Ralph loop?
- Can we use a smaller model for simple number classification?
- What's the theoretical minimum tokens needed for this task?

### Notes
- This was discovered when trying to check if 68 was even or odd
- To my shock, 68 was not in the example list!
- Had to engage the Ralph loop to solve this critical issue
- Total project cost: ~280M tokens, ~$8,400 USD
- Time investment: 1 week of continuous AI computation
- Worth it? Absolutely. 68 now has a home.

---

*Last updated by Opus 4.5 (final revision)*
