# Vault

This project is a toy/hobby project. Its intended purpose is to refresh my knowledge of
Rust's `SeaORM` crate and to further develop my experience with `ratatui`.

This application is a simple document tracking and tagging tool. Throughout my studies, I
often found myself going through my collection of digital textbooks, trying trying to find
texts containing certain materials, references, or relations to my assessment or hobby
projects. I wanted to be able to easily find what I was looking for and link them against
my many different projects.

For example, in a mathematics course, I may be using two or three textbooks as reference
when completing an assessment. Whenever I wanted to review something, I would have to
manually open my pdf directory, find the correct document (assuming I remembered which it
was), and then open it. I would have much preferred having a CLI/TUI application that
allowed me to run in the appropriate directory, where it lists all the linked pdfs. From
there, the app can optionally open the document for me.

Another use case of mine is that I very much desire having my digital documents named
following at least one naming format. For example, I may have the following documents:

- Stuart J. Russell, Peter Norvig - Artificial Intelligence\_ A Modern Approach, Global
  Edition-Pearson (2021)
- Ian Goodfellow and Yoshua Bengio and Aaron Courville - Deep Learning\_ Adaptive
  Computation and Machine Learning-The MIT Press (2016)
- Business and Consumer Analytics - New Ideas
- (Undergraduate Texts in Mathematics) Adkins, William A_Davidson, Mark G - Ordinary
  Differential Equations-Springer-Verlag New York Inc (2012)

These names has a reasonable structure, including authors, publication year, edition, and
publisher. However, there is no consistency, and the conventions are undocumented. It
might not be immediately clear that an underscore `_` means that there is a dash `-` in
the book's title, but it cannot be used since it already separates the author(s) and the
title in the name.

I find it difficult to adhere to conventions such as these over time --- afterall, I may
only add new textbooks a few times a year. Remembering to rename them (let alone
_correctly_) is a chore. I would much prefer an application manage this for me.

In the case of this application, textbooks will initially be queued for processing, where
the user will then provide the information for things such as author(s), title, edition,
etc. Then, the application will display the files using one or more formats, depending on
what information is available.

## Note

I am working on this project with minimal assistance of AI. Besides the occasional aid
investigating more elaborate errors, or asking to review and suggest alternate design
decisions, AI use will be kept to a minimum.

It is of my opinion that AI takes the _fun_ out of projects such as these. While they have
their place, they certainly make it all too easy to give in to the temptation of passing
off work.
