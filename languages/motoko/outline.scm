(line_comment) @annotation
(block_comment) @annotation
(doc_comment) @annotation

; Top level
(source_file
    (obj_dec
        ["module" "object" ("persistent"? "actor")] @context
        (identifier)? @name
    ) @item
)

(source_file
    (class_dec
        ("persistent"? "actor")? @context
        "class" @context
        (type_identifier)? @name
    ) @item
)

(source_file
    (func_dec
        [("shared" "query"?) ("composite"? "query")]? @context
        "func" @context
        name: (identifier) @name
        return_ty: (typ_annot (async_typ "async" @context)?)?
    ) @item
)

(source_file
    (typ_dec
        "type" @context
        (type_identifier) @name
    ) @item
)

(source_file
    (let_dec
        "let" @context
        (annot_pat
            (var_pat
                (identifier) @name
            )
            (typ_annot
                (func_typ
                    (tup_typ
                        "(" @context
                        ")" @context
                    )
                    [(async_typ "async" @context) (tup_typ) (path_typ)]
                )
            )
        )
    ) @item
)

(source_file
    (var_dec
        "var" @context
        (var_pat
            (identifier) @name
        )
        (typ_annot
            (func_typ
                (tup_typ
                    "(" @context
                    ")" @context
                )
                [(async_typ "async" @context) (tup_typ) (path_typ)]
            )
        )
    ) @item
)

; Object
(dec_field
    ["private" "public" "system"]? @context
    ["flexible" "stable" "transient"]? @context
    (obj_dec
        ["module" "object" ("persistent"? "actor")] @context
        (identifier)? @name
    )
) @item

(block_exp
    (obj_dec
        ["module" "object" ("persistent"? "actor")] @context
        (identifier) @name
    ) @item
)

; Class
(dec_field
    ["private" "public"]? @context
    (class_dec
        "class" @context
        (type_identifier) @name
    )
) @item

(block_exp
    (class_dec
        "class" @context
        (type_identifier) @name
    ) @item
)

; Type
(dec_field
    ["private" "public" "system"]? @context
    (typ_dec
        "type" @context
        (type_identifier) @name
    )
) @item

; Variants
(variant_typ
    (typ_tag
        (tag_identifier) @name
    ) @item
)

(variant_typ
    (typ_tag
        (typ_annot
            (obj_typ
                (val_tf
                    "var"? @context
                    (identifier) @name
                ) @item
            )
        )
    )
)

; Function
(dec_field
    ["private" "public" "system"]? @context
    ["flexible" "stable" "transient"]? @context
    (func_dec
        [("shared" "query"?) ("composite"? "query")]? @context
        "func" @context
        name: (identifier) @name
        return_ty: (typ_annot (async_typ "async" @context)?)?
    )
) @item

(dec_field
    ["private" "public" "system"]? @context
    (let_dec
        "let" @context
        (annot_pat
            (var_pat (identifier) @name)
            (typ_annot
                (func_typ
                    (tup_typ
                        "(" @context
                        ")" @context
                    )
                    [(async_typ "async" @context) (tup_typ) (path_typ)]
                )
            )
        )
    )
) @item

(block_exp
    (func_dec
        [("shared" "query"?) ("composite"? "query")]? @context
        "func" @context
        name: (identifier) @name
        return_ty: (typ_annot (async_typ "async" @context)?)?
    ) @item
)

(block_exp
    (var_dec
        "var" @context
        (var_pat (identifier) @name)
        (typ_annot
            (func_typ
                (tup_typ
                    "(" @context
                    ")" @context
                )
                [(async_typ "async" @context) (tup_typ) (path_typ)]
            )
        )
    ) @item
)

(block_exp
    (let_dec
        "let" @context
        (annot_pat
            (var_pat (identifier) @name)
            (typ_annot
                (func_typ
                    (tup_typ
                        "(" @context
                        ")" @context
                    )
                    [(async_typ "async" @context) (tup_typ) (path_typ)]
                )
            )
        )
    ) @item
)

; Member Variable
(obj_body
    (dec_field
        ["private" "public" "system"]? @context
        ["flexible" "stable" "transient"]? @context
        [
            (let_dec
                "let" @context
                (var_pat (identifier) @name)
            )
            (var_dec
                "var" @context
                (var_pat (identifier) @name)
                (typ_annot
                    (func_typ
                        (tup_typ
                            "(" @context
                            ")" @context
                        )
                        [(async_typ "async" @context) (tup_typ) (path_typ)]
                    )?
                )?
            )
        ]
    ) @item
)

(object_exp
    (exp_field
        "var"? @context
        (identifier) @name ;
    ) @item
)
