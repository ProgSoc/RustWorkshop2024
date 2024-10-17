# Rust Workshop 2024

Repository of example programs for ProgSoc's Rust Workshop 2024.

## Instructions

Follow along with the slides in the workshop
with Replit so that you can conveniently switch over
to test small snippets if you'd like.

When we get to a slide that says **"Example Program"**,
it will come with a bracket (e.g., **"Example Program (Part 1)"**).
Please navigate to the `src/parts` folder for the corresponding template code
(in this example, `src/parts/part_1.rs`).
Copy and paste the code into your Replit workspace
and implement the functionalities as prompted by each `TODO`.

If you get stuck or the workshop is moving a bit faster than you'd like,
there's always the corresponding solution code for each part in the
`src/solutions` folder, with comments about what has changed since the previous part.

## After the Workshop

Funnily enough, the Rust package that this repository contains
actually runs as-is. For fun, you can download the code onto your local computer
and, if you have Rust installed, `cargo run` it.
You can even test out different parts of the workshop code by changing the `part` variable
(keep it an integer from 1 to 5 though),
and even check your own implemented output against the workshop solution code
by toggling the `use_solution` boolean variable.