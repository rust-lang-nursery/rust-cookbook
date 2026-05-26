# File System

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Read a whole file into a string][ex-read-to-string] | [![std-badge]][std] [![tempfile-badge]][tempfile] | [![cat-filesystem-badge]][cat-filesystem] |
| [Write a string to a file][ex-write-string] | [![std-badge]][std] [![tempfile-badge]][tempfile] | [![cat-filesystem-badge]][cat-filesystem] |
| [Read lines of strings from a file][ex-std-read-lines] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Rename or atomically replace a file][ex-rename] | [![std-badge]][std] [![tempfile-badge]][tempfile] | [![cat-filesystem-badge]][cat-filesystem] |
| [Avoid writing and reading from a same file][ex-avoid-read-write] | [![same_file-badge]][same_file] | [![cat-filesystem-badge]][cat-filesystem] |
| [Access a file randomly using a memory map][ex-random-file-access] | [![memmap-badge]][memmap] | [![cat-filesystem-badge]][cat-filesystem] |
| [Construct and inspect a path][ex-path-inspect] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] |
| [Create a nested directory tree][ex-create-dir-all] | [![std-badge]][std] [![tempfile-badge]][tempfile] | [![cat-filesystem-badge]][cat-filesystem] |
| [File names that have been modified in the last 24 hours][ex-file-24-hours-modified] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] [![cat-os-badge]][cat-os] |
| [Find loops for a given path][ex-find-file-loops] | [![same_file-badge]][same_file] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively find duplicate file names][ex-dedup-filenames] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively find all files with given predicate][ex-file-predicate] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Traverse directories while skipping dotfiles][ex-file-skip-dot] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Recursively calculate file sizes at given depth][ex-file-sizes] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem] |
| [Find all png files recursively][ex-glob-recursive] | [![glob-badge]][glob] | [![cat-filesystem-badge]][cat-filesystem] |
| [Find all files with given pattern ignoring filename case][ex-glob-with] | [![glob-badge]][glob] | [![cat-filesystem-badge]][cat-filesystem] |

[ex-avoid-read-write]: file/read-write.html#avoid-writing-and-reading-from-a-same-file
[ex-create-dir-all]: file/dir.html#create-a-nested-directory-tree
[ex-dedup-filenames]: file/dir.html#recursively-find-duplicate-file-names
[ex-file-24-hours-modified]: file/dir.html#file-names-that-have-been-modified-in-the-last-24-hours
[ex-file-predicate]: file/dir.html#recursively-find-all-files-with-given-predicate
[ex-file-sizes]: file/dir.html#recursively-calculate-file-sizes-at-given-depth
[ex-file-skip-dot]: file/dir.html#traverse-directories-while-skipping-dotfiles
[ex-find-file-loops]: file/dir.html#find-loops-for-a-given-path
[ex-glob-recursive]: file/dir.html#find-all-png-files-recursively
[ex-glob-with]: file/dir.html#find-all-files-with-given-pattern-ignoring-filename-case
[ex-path-inspect]: file/dir.html#construct-and-inspect-a-path
[ex-random-file-access]: file/read-write.html#access-a-file-randomly-using-a-memory-map
[ex-read-to-string]: file/read-write.html#read-a-whole-file-into-a-string
[ex-rename]: file/read-write.html#rename-or-atomically-replace-a-file
[ex-std-read-lines]: file/read-write.html#read-lines-of-strings-from-a-file
[ex-write-string]: file/read-write.html#write-a-string-to-a-file

{{#include links.md}}
