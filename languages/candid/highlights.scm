; Keywords
[
  "func"
  "opt"
  "principal"
  "record"
  "service"
  "type"
  "variant"
  "vec"
] @keyword

(funcann) @keyword

[
  (nat)
] @number

(text) @string

["->" "=" ":"] @operator

["(" ")" "{" "}"] @punctuation.bracket
["," ";"] @punctuation.delimiter

; Comments
(comment) @comment

; Types
[
  "bool"
  "text"
  "reserved"
  "empty"
  "principal"
  "blob"
] @type

(numtype) @type

(datatype (id) @type)

(def
  (id) @type
  (datatype))

(methtype
  name: (name (id) @function))

(constype
  "record"
  (fieldtype
    (name (id) @parameter)?
    hasharg: (name (id) @parameter)?))

(constype
  "variant"
  (fieldtype
    (name (id) @tag)?
    hasharg: (name (id) @tag)?))

(actor
  name: (id) @type.special)
