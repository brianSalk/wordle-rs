## First off, thank you so much for even reading this!  I give all contributors credit no matter how trivial the contribution.  
This project is full of low hanging fruit for developers and non-developers alike, here are some contributions that I would greatly appreciate.  
* fix spelling errors and grammatical errors in **any** of this projects documentation.  
* improve the readability of the rust code  
* add/remove words from the word list(s) that do not belong (uncommon words, plurals etc.)  
* figure out a better way to create a global `wordle` command, right now I have the `setup.sh` file, but there should be a better way to do it  
* write/improve comments in the code  
* Write test cases for the program, as of now I have none but tests are always good to have.
* add a new language to the `LanguageStuff` struct or improve the german/english version.  

## removing plurals/unwanted words
removing plurals from the five_upper.txt file is probably the easiest thing to do.  I have a tool here called i-rep[https://github.com/brianSalk/i-rep.git] that allows you to interactivly delete lines from a file.  It is best to use an interactive tool because many non-plural english words end with 's' (abyss, amiss, grass, etc.) so you don't want to just delete words that end in 's'.  If you do use my tool to remove plurals, place it in a directory in your PATH variable and run `i-rep five_upper.txt ".*S$"` then each time you find a word you want to delete just type `d` for delete.  when you are using the tool, you can simply `ctl-c` out and all your changes will still be there.  
As for unwanted words, this is subjective.  Your best bet is to submit a pull request removing very few words as I will be more likely to accept such pull requests.  This can be done for any language

## language specific help
if you are improving the quality of one language, go to the method with that name in the LanguageStuff struct and make all modifications there directly.  I would love to have a native speaker improve the german stuff as I have only a tenuous grasp of the language.
## code comments and documentation
If you want to add comments to the code that would be great.  Go ahead and use chatGPT or whatever other tools you have access to.
