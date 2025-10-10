(line_comment) @annotation
(block_comment) @annotation
(doc_comment) @annotation

; Object
(
  ["private" "public" "system"]? @context
  ["flexible" "stable" "transient"]? @context
  (obj_dec
    ["module" "object" ("persistent"? "actor")] @context
    (identifier)? @name) @item)

(obj_typ
  (val_tf
    "var"? @context
    (identifier) @name) @item)

; Class
(
  ["private" "public" "system"]? @context
  (class_dec
    ("persistent"? "actor")? @context
    "class" @context
    (type_identifier) @name) @item)

; Type
(
  ["private" "public" "system"]? @context
  (typ_dec
    "type" @context
    (type_identifier) @name) @item)

; Variants
(variant_typ
  (typ_tag
    (tag_identifier) @name) @item)

; Function
(
  ["private" "public" "system"]? @context
  ["flexible" "stable" "transient"]? @context
  (func_dec
    [("shared" "query"?) ("composite"? "query")]? @context
    "func" @context
    name: (identifier) @name
    return_ty: (typ_annot
      (":" @context (async_typ "async" @context))?)?) @item)

(
  ["private" "public" "system"]? @context
  ["flexible" "stable" "transient"]? @context
  (let_dec
    "let" @context
    (annot_pat
      (var_pat (identifier) @name)
      (typ_annot
        (func_typ
          (tup_typ
            "(" @context
            ")" @context)
          "->"
          (async_typ "async" @context)?)))) @item)

(
  ["private" "public" "system"]? @context
  ["flexible" "stable" "transient"]? @context
  (var_dec
    "var" @context
    (var_pat (identifier) @name)
    (typ_annot
      (func_typ
        (tup_typ
          "(" @context
          ")" @context)
        "->"
        (async_typ "async" @context)?))) @item)
