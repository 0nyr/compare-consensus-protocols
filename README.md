# Compare consensus protocols

> State of the art overview of the different consensus algorithms used by blockchains / cryptocurrencies.

### Useful links

##### tools

[bibliography manager | bibguru](https://github.com/0nyr/compare-consensus-protocols) ⭐️

##### technical report (TR)

[TR: What is it &amp; How to Write it?](https://blog.bit.ai/technical-report/) ⭐️

#### latex

##### glossaries & acronyms

[how to make Glossaries/Acronyms in LaTeX](https://www.resurchify.com/latex_tutorial/latex_glossaries.php)

[Glossaries / Acronyms | Overleaf](https://www.overleaf.com/learn/latex/Glossaries)

[List of glossaries not displaying](https://tex.stackexchange.com/questions/192378/list-of-glossaries-not-displaying)

### presentation

[Presentation on Canva](https://www.canva.com/design/DAFC139vlco/JKfnzCtc0PL6PJiq1qAWUw/view?utm_content=DAFC139vlco&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton)

## Notes

Below is the list of the best known algos.

* [X] PoW
* [X] PoS
* [X] PoSpace & PoCapacity
* [X] PoET
* [ ] DPoS
* [ ] DPoR
* [ ] Hybrid PoW + PoS
* [ ] CryptoNight/CryptoNote
* [ ] Equihash
* [ ] NeoScrypt
* [ ] Quark
* [ ] Scrypt
* [ ] DBft

## commands

### latex command

`biber main` - build bibliography entries

`makeglossaries main` - build acronyms and glossary entries

`pdflatex main.tex` - build pdf from main latex file.

## VSCode

##### unable to get `rls`

VScode has problem with rust `nightly`. Read [this | GitHub issue](https://github.com/rust-lang/vscode-rust/issues/237#issuecomment-359639894). The fix is rather simple, just set `"rust-client.channel": "stable"` in VS Code user settings.
