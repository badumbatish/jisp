(lang dune 3.15)

(name compiler)

(generate_opam_files true)

(source
 (github badumbatish/jisp))

(authors "Jasmine Tang")

(maintainers "Jasmine Tang")

(license LICENSE)

(documentation https://url/to/documentation)

(package
 (name compiler)
 (synopsis "Lisp compiler")
 (description "A lisp compiler in progress")
 (depends ocaml dune)
 (tags
  (topics "to describe" your project)))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/dune-files.html#dune-project
