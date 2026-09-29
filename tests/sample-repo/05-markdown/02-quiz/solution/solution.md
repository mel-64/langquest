---
title    = "HTTP Status Codes Quiz"
hints    = [
    "4xx codes mean the client made a mistake.",
    "A missing page is always reported with the same famous code.",
    "Answers: 1B, 2A, 2C, 3A, 3C.",
]
# Every keyword anchors on the unique option label (`1A`..`3D`).
keywords = [
    # Correct answers ticked (1B, 2A, 2C, 3A, 3C):
    "s/(?m)^1B\\s*\\[\\s*x\\s*\\]/",
    "s/(?m)^2A\\s*\\[\\s*x\\s*\\]/",
    "s/(?m)^2C\\s*\\[\\s*x\\s*\\]/",
    "s/(?m)^3A\\s*\\[\\s*x\\s*\\]/",
    "s/(?m)^3C\\s*\\[\\s*x\\s*\\]/",
    # Wrong options must stay unticked (1A, 1C, 1D, 2B, 2D, 3B, 3D):
    "s/(?m)^1A\\s*\\[\\s*\\]/",
    "s/(?m)^1C\\s*\\[\\s*\\]/",
    "s/(?m)^1D\\s*\\[\\s*\\]/",
    "s/(?m)^2B\\s*\\[\\s*\\]/",
    "s/(?m)^2D\\s*\\[\\s*\\]/",
    "s/(?m)^3B\\s*\\[\\s*\\]/",
    "s/(?m)^3D\\s*\\[\\s*\\]/",
]
---

## Solution

1. **B** — `404 Not Found`: the client asked for a page that does not exist.
2. **A, C** — `404 Not Found` is a 4xx client error, `500` a 5xx server error.
3. **A, C** — `201 Created` and `200 OK` are both 2xx success codes.

