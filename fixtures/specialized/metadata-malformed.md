---
description: >
  this is a long
  paragraph value spread
  across multiple lines
nested:
  key1: value1
  key2: value2
title: Malformed Sample
---

Body. The frontmatter above mixes a folded scalar and a nested mapping —
shapes that the line-based heuristic cannot fully parse. It should still
extract `description` and `title` while skipping the continuation lines,
and the document should not break.
