(comment) @annotation

; Type
(def
  "type" @context
  (id) @name) @item

(actor
  "service" @context
  name: (id)? @name) @item

(methtype
  name: (name)? @name
  type: (functype (funcann) @context)?) @item

(constype
  ["record" "variant"]
  (fieldtype
    (name) @name) @item)
