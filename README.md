# KnitLang

This is a first go at a standardized human-readable knitting pattern notation language. It is heavily inspired by [CookLang](https://github.com/cooklang).

My motivation to create this was mostly that I was really struggling with keeping track of counting rows in knitting projects. While it's possible to just use a manual counter or a simple counting app on your phone or even just taking notes, this process seems awfully cumbersome to me. In the case of more complex patterns I was also really struggling to keep track of which part of the pattern I was currently working on. It's just so easy to confuse lines in PDFs that always look different from designer to designer.

It's also very obvious that there is already a loose konvention of knitting pattern algorithms(K1 for knitting one stitch, P1 for purling one stitch, k2tog for knitting two together). There's also several notations for looping certain procedures.

So it seemed obvious to me that this is already very close to a scripting language. So why not just use the current conventions, adapt them a little, and just use them as an actual scripting language?!
Patterns can then be saved as the algorithms that they really are! And they can easily be imported into applications specifically designed for counting rows/stitches!

And that would be awesome! The app could not only show you the number of your current row, but even the specific instructions for that row. And even better: it can calculate and display the number of stitsches you should have on your needle at any step so you can check if you made any mistakes!

So here is my attempt at creating such a scripting language.
(Note that this is not intended for knitting machines. Such scripting languages already exist, but are not human readable. This is specifically meant for manual (hand-)knitting.)

## The Manifest

All knitting shorthands that I have personally come across start with few letters indicating the instruction itself. This could be a spicifc stitch, increase, decrease, stitch slip, marker positioning, etc. In very few cases this instruction contains a number itself (e.g. k2tog). If the instruction ends with a number, this is the multiplier that indicates how often the stitch should be performed. If it does not end with a number it is assumed to be performed once.
This pattern is unambigous so I'm adoopting it as is.
Some patterns include instructions like knit until 3 stitches are left on the needle, or knit to the next marker. So instead of with a number, an instruction can also end with a question mark. If you are supposed to knit to the next marker you can just add a to-marker instruction afterwards. If you should leave something like three stitches before the end of the row, then there is typically instructions of what to do with those leftover stitches so the instruction is again unambiguous.
Each instruction should be separated by a white space.

A new knitted row starts with a new line in the script - simple.

For loops I'm introducing a new pattern, since the currently used patterns are, in my opinion, often a bit confusing and ambiguous. So I'm just using curly brackets. If the curly brackets are inline, then the loop is just accross different types of stitches in the same row. If the brackets are placed in separate lines than they are interpreted as repeating rows. Similar to single instruction a number directly after the closing bracket is used as the multiplier of how often the row should be repeated.

So this is an example which is the well known Sophie Scarf:

```
co6
{
    {
        k? slyf3
    }7
    k2 kfb k? slyf3
}22
{
    {
        k? slyf3
    }7
    k3 skp k? slyf3
}21
{
    k? slyf3
}7
k2 skp slyf3
{
    k? slyf3
}5
bol3 bor3
```

## Example Code

This Project currently contains code for a rust CLI that takes in a .knit file and returns a very simple CLI counter app for that file.

It also contains a very simple demo JS counter app that displays a counter application for the Sophie Scarf

## Roadmap

This is a collections of Steps that should be done for continued development.

- Common Rust Parser
    - A Rust parser that can be used as a library. Ideally it should have its own abstract syntax tree (AST). It should then be able to return things like readable Row arrays, an array of stitch numbers etc. It could the also be used from JS through WebAssembly.
- A (relatively) complete collection of instructions that can be parsed
- WebApp:
    - Ability to import .knit files
    - some kind of statefulness to remember past patterns and progress
- headers:
    - photos
    - yarn info
    - swatch info
    - size info
    - progress or list of progresses
- better number of stitches on needles. This could include numbers between markers.
- inferring number of stitches if question marks are used (this might be tricky), also actually "to marker" is often better than a large number of stitches
- error checking and useful logic issue messages.

I think it makes sense to include markers just as pseudo-instructions. I think a new complex "knit-to-marker" defintion would be problematic. Anyways tracking the number of stitches in subsections is gonna be hard. It heavily depends on correctness of patterns.
