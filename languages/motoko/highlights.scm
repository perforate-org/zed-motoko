; Keywords
[
  "import"
  "actor"
  "module"
  "object"
  "class"
  "type"
  "private"
  "public"
  "system"
  "flexible"
  "stable"
  "transient"
  "shared"
  "async"
  "async*"
  "await"
  "await*"
  "query"
  "if"
  "else"
  "switch"
  "case"
  "func"
  "return"
  "let"
  "var"
  "for"
  "while"
  "loop"
  "break"
  "continue"
  "assert"
  "ignore"
  "try"
  "catch"
  "throw"
  "and"
  "or"
  "not"
  "with"
  "composite"
  "label"
  "debug"
  "from_candid"
  "to_candid"
  "prim"
  "debug_show"
  "do"
  "persistent"
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
(doc_comment) @comment.doc

; Functions
(func_dec
  name: (identifier) @function)

; Types
[
  (type_identifier)
  (typ_path)
  (prim_typ)
] @type

; Variables
[
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

[
  "+" "-" "*" "/" "%" "**" "+%" "*%" "**%" "&" "|" "^" "<<" ">>" "<<>" "<>>" "->"
  "#" "==" "!=" "<" "<=" ">" ">=" "+=" "-=" "*=" "**=" "/=" "%=" "+%=" "-%=" "*%=" "="
  "**%=" "&=" "|=" "^=" "<<=" ">>=" "<<>=" "<>>=" "@=" "|>" "?" "and" "or" "not" "!" ":="
] @operator

; Tags
(tag_identifier) @tag

; Punctuation
["(" ")" "[" "]" "{" "}" "<" ">"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
["#" "<:"] @punctuation.special

(identifier) @variable

; CamelCase for classes
((identifier) @type.class
  (#match? @type.class "^_*[A-Z][A-Za-z0-9_]*$"))
