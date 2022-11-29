# The Supervisionary proof-checking system

Supervisionary is an experimental proof-checking system for Gordon's higher-order logic ("HOL").
Rather than using programming language features to isolate and protect the kernel, as in typical LCF-style implementations of HOL like HOL4 and Isabelle, Supervisionary uses *privelege* akin to how an operating system kernel is isolated and protected from untrusted user-space code.

At this point, fully exploring this idea is still a work-in-progress.

For more information, see the `paper/prisc22/prisc22.tex` two-page abstract, or a pre-built [PDF](https://dominicpm.github.io/publications/mulligan-supervisionary-2022.pdf), of our accepted PriSC 2022 talk on Supervisionary and which relates the main ideas behind the system.
