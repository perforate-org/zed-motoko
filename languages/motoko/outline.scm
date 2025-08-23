(line_comment) @annotation

(dec_field
    ["private" "public" "system"]? @context
    ["flexible" "stable" "transient"]? @context
    (func_dec
        [("shared" "composite"? "query"?) ("shared"? "composite"? "query")]? @context
        "func" @context
        name: (identifier) @name)) @item

(class_dec
    [("shared" "composite"? "query"?) ("shared"? "composite"? "query")]? @context
    ["object" "module" ("persistent"? "actor")]? @context
    "class" @context
    (type_identifier) @name) @item

(obj_dec
    ["module" "object" ("persistent"? "actor")] @context
    (identifier)? @name) @item
