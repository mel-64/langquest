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
    "s/(?m)^-\\s*\\[\\s*x\\s*\\]\\s*1B/",
    "s/(?m)^-\\s*\\[\\s*x\\s*\\]\\s*2A/",
    "s/(?m)^-\\s*\\[\\s*x\\s*\\]\\s*2C/",
    "s/(?m)^-\\s*\\[\\s*x\\s*\\]\\s*3A/",
    "s/(?m)^-\\s*\\[\\s*x\\s*\\]\\s*3C/",
    # Wrong options must stay unticked (1A, 1C, 1D, 2B, 2D, 3B, 3D):
    "s/(?m)^-\\s*\\[\\s*\\]\\s*1A/",
    "s/(?m)^-\\s*\\[\\s*\\]\\s*1C/",
    "s/(?m)^-\\s*\\[\\s*\\]\\s*1D/",
    "s/(?m)^-\\s*\\[\\s*\\]\\s*2B/",
    "s/(?m)^-\\s*\\[\\s*\\]\\s*2D/",
    "s/(?m)^-\\s*\\[\\s*\\]\\s*3B/",
    "s/(?m)^-\\s*\\[\\s*\\]\\s*3D/",
]
---

## Solution

1. **B** — `404 Not Found`: the client asked for a page that does not exist.
2. **A, C** — `404 Not Found` is a 4xx client error, `500` a 5xx server error.
3. **A, C** — `201 Created` and `200 OK` are both 2xx success codes.

