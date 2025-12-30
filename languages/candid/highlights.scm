; Keywords
[
  "func"
  "import"
  "opt"
  "principal"
  "record"
  "service"
  "type"
  "variant"
  "vec"
] @keyword

(funcann) @keyword

((id) @keyword
  (#match? @keyword
    "^(func|opt|principal|record|service|type|variant|vec|oneway|query|composite_query)$"))

[
  (nat)
  (int)
  (float)
] @number

(text) @string

[
  "->"
  "="
  ":"
] @operator

[
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  ";"
] @punctuation.delimiter

; Literals
(bool_literal) @boolean

(null_literal) @constant

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

(datatype
  (id) @type)

(def
  (id) @type
  (datatype))

; Assume uppercase names are types
((id) @type
  (#match? @type "^[A-Z]"))

((id) @type
  (#match? @type
    "^(bool|text|reserved|empty|principal|blob|nat|nat8|nat16|nat32|nat64|int|int8|int16|int32|int64|float32|float64)$"))

(methtype
  name: (name
    (id) @function))

(constype
  "record"
  (fieldtype
    (name
      (id) @parameter)?
    hasharg: (name
      (id) @parameter)?))

(constype
  "variant"
  (fieldtype
    (name
      (id) @tag)?
    hasharg: (name
      (id) @tag)?))

(actor
  name: (id) @type.special)
