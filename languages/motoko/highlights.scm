; Keywords
[
  "actor"
  "and"
  "assert"
  "async"
  "async*"
  "await"
  "await*"
  "break"
  "case"
  "catch"
  "class"
  "composite"
  "continue"
  "debug"
  "debug_show"
  "do"
  "else"
  "finally"
  "flexible"
  "for"
  "from_candid"
  "func"
  "if"
  "in"
  "ignore"
  "import"
  "label"
  "let"
  "loop"
  "mixin"
  "module"
  "not"
  "object"
  "or"
  "persistent"
  "prim"
  "private"
  "public"
  "query"
  "return"
  "shared"
  "stable"
  "switch"
  "system"
  "throw"
  "to_candid"
  "transient"
  "try"
  "type"
  "var"
  "weak"
  "while"
  "with"
] @keyword

; Literals
(bool_literal) @boolean
(null_literal) @constant

[
  (int_literal)
  (hex_literal)
  (float_literal)
] @number

(text_literal) @string
(char_literal) @string.special

; Comments
(line_comment) @comment
(block_comment) @comment
(comment_text) @comment
(doc_comment) @comment.doc

; Types
[
  (type_identifier)
  (typ_path)
  (prim_typ)
] @type

; Variables
[
  (identifier)
  (var_exp)
  (var_pat)
] @variable

; Operators
[
  (bin_op)
  (unop)
  (binassign_op)
  (unassign_op)
  (rel_op)
] @operator

["->" "=" "<:" "?" ":" "<" ">"] @operator

(bang_exp_block "!" @operator)
(bang_exp_object "!" @operator)

(assign_exp_block ":=" @operator)
(assign_exp_object ":=" @operator)

(not_exp "not" @operator)

; Punctuation
["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";"] @punctuation.delimiter

; Functions
(func_dec
  name: (identifier) @function)

(_
  (var_pat (identifier) @function)
  (typ_annot (func_typ)))

; Properties
(dot_exp_block
  (identifier) @property)

(dot_exp_object
  (identifier) @property)

; Identifies function calls followed by dot notation as method calls
(call_exp_block
  (dot_exp_block
    (identifier) @function.method)
    (par_exp))

(call_exp_object
  (dot_exp_object
    (identifier) @function.method)
    (par_exp))

; Normal function call
(call_exp_block
  (var_exp) @function
  (par_exp)
  (#not-eq? @function "async"))

(call_exp_object
  (var_exp) @function
  (par_exp)
  (#not-eq? @function "async"))

; Highlight async as keyword in call expressions
(call_exp_block
  (var_exp) @keyword
  (#eq? @keyword "async"))

(call_exp_object
  (var_exp) @keyword
  (#eq? @keyword "async"))

; Tuple projection
(proj_identifier) @number

; Type parameters
(inst "system"? @type.special)
(typ_params "system"? @type.special)
(typ_bind
  name: (type_identifier) @type
  (_
    "<:"
    supertype: (type_identifier) @type)?)

; Assume uppercase names are types
((identifier) @type
  (#match? @type "^[A-Z]"))

; Tags
(tag_identifier
  "#" @tag
  (identifier) @tag)

; Record properties
(object_exp
  (exp_field (identifier) @property))

(obj_typ
  (val_tf (identifier) @property))
