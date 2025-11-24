; functions
(func_dec
  body: (_
    "{"
    (_)* @function.inside
    "}" )) @function.around

; classes
(class_dec
  (obj_body) @class.inside
  ) @class.around

(obj_dec
  (obj_body) @class.inside
  ) @class.around

(typ_dec
  (obj_typ) @class.inside
  ) @class.around

(typ_dec
  (variant_typ) @class.inside
  ) @class.around

; comments
(line_comment)+ @comment.around

(block_comment) @comment.around
