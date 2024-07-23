# ! [cfg_attr (rustfmt , rustfmt_skip)] # ! [allow (bad_style , missing_docs , unreachable_pub , unused)] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [repr (u16)] pub enum TSKind { anon_end = 0 , identifier = 1 , anon_LPAREN = 2 , anon_RPAREN = 3 , anon_BANG = 4 , anon_TILDE = 5 , anon_DASH = 6 , anon_PLUS = 7 , anon_AMP = 8 , anon_DOT_DOT_DOT = 9 , anon_STAR = 10 , anon_SLASH = 11 , anon_PERCENT = 12 , anon_PIPE_PIPE = 13 , anon_AMP_AMP = 14 , anon_PIPE = 15 , anon_CARET = 16 , anon_EQ_EQ = 17 , anon_BANG_EQ = 18 , anon_GT = 19 , anon_GT_EQ = 20 , anon_LT_EQ = 21 , anon_LT = 22 , anon_LT_LT = 23 , anon_GT_GT = 24 , anon_GT_GT_GT = 25 , anon_POUNDinclude = 26 , anon_POUNDtryinclude = 27 , anon_POUNDdefine = 28 , anon_COMMA = 29 , macro_param = 30 , anon_POUNDundef = 31 , anon_POUNDif = 32 , anon_POUNDelseif = 33 , anon_POUNDassert = 34 , anon_defined = 35 , preproc_else = 36 , preproc_endif = 37 , preproc_endinput = 38 , anon_POUNDpragma = 39 , anon_POUNDerror = 40 , anon_POUNDwarning = 41 , anon_using__intrinsics__DOTHandle = 42 , anon_assert = 43 , anon_static_assert = 44 , anon_EQ = 45 , anon_forward = 46 , anon_native = 47 , alias_operator = 48 , anon_operator = 49 , anon_COLON = 50 , anon_const = 51 , anon_public = 52 , anon_stock = 53 , anon_static = 54 , anon_new = 55 , anon_decl = 56 , anon_enum = 57 , anon_PLUS_EQ = 58 , anon_DASH_EQ = 59 , anon_STAR_EQ = 60 , anon_SLASH_EQ = 61 , anon_PIPE_EQ = 62 , anon_AMP_EQ = 63 , anon_CARET_EQ = 64 , anon_TILDE_EQ = 65 , anon_LT_LT_EQ = 66 , anon_GT_GT_EQ = 67 , anon_LBRACE = 68 , anon_RBRACE = 69 , anon_struct = 70 , anon_typedef = 71 , anon_typeset = 72 , anon_function = 73 , anon_funcenum = 74 , anon_functag = 75 , anon_methodmap = 76 , anon___nullable__ = 77 , anon_property = 78 , anon_get = 79 , anon_set = 80 , anon_LBRACK = 81 , anon_RBRACK = 82 , anon_void = 83 , anon_bool = 84 , anon_int = 85 , anon_float = 86 , anon_char = 87 , anon__ = 88 , anon_Float = 89 , anon_String = 90 , any_type = 91 , anon_for = 92 , anon_while = 93 , anon_do = 94 , anon_break = 95 , anon_continue = 96 , anon_if = 97 , anon_else = 98 , anon_switch = 99 , anon_case = 100 , anon_COLON_ = 101 , anon_default_ = 102 , anon_return_ = 103 , anon_delete_ = 104 , anon__manual_semicolon_ = 105 , anon_GT_GT_GT_EQ_ = 106 , anon_PERCENT_EQ_ = 107 , anon_DOT_ = 108 , anon_QMARK_ = 109 , anon_COLON_COLON_ = 110 , anon_DASH_DASH_ = 111 , anon_PLUS_PLUS_ = 112 , anon_sizeof_ = 113 , anon_view_as_ = 114 , int_literal = 115 , float_literal = 116 , anon_SQUOTE_ = 117 , character = 118 , anon_DQUOTE_ = 119 , anon_string_literal_token1_ = 120 , escape_sequence = 121 , bool_literal = 122 , null = 123 , this = 124 , system_lib_string = 125 , comment = 126 , anon__automatic_semicolon_ = 127 , anon__ternary_colon_ = 128 , preproc_arg = 129 , source_file = 130 , anon__preproc_expression_ = 131 , preproc_parenthesized_expression = 132 , preproc_unary_expression = 133 , preproc_binary_expression = 134 , preproc_include = 135 , preproc_tryinclude = 136 , preproc_macro = 137 , preproc_define = 138 , preproc_undefine = 139 , preproc_if = 140 , preproc_elseif = 141 , preproc_assert = 142 , preproc_defined_condition = 143 , preproc_pragma = 144 , preproc_error = 145 , preproc_warning = 146 , hardcoded_symbol = 147 , assertion = 148 , function_definition = 149 , function_declaration = 150 , function_declaration_kind = 151 , parameter_declarations = 152 , parameter_declaration = 153 , rest_parameter = 154 , alias_declaration = 155 , alias_assignment = 156 , global_variable_declaration = 157 , variable_declaration_statement = 158 , variable_storage_class = 159 , visibility = 160 , variable_declaration = 161 , dynamic_array_declaration = 162 , dynamic_array = 163 , new_expression = 164 , old_global_variable_declaration = 165 , old_variable_declaration_statement = 166 , old_for_loop_variable_declaration_statement = 167 , old_variable_declaration = 168 , r#enum = 169 , enum_entries = 170 , enum_entry = 171 , enum_struct = 172 , enum_struct_field = 173 , enum_struct_method = 174 , typedef = 175 , typeset = 176 , typedef_expression = 177 , funcenum = 178 , funcenum_member = 179 , functag = 180 , methodmap = 181 , methodmap_alias = 182 , methodmap_native = 183 , methodmap_native_constructor = 184 , methodmap_native_destructor = 185 , methodmap_method = 186 , methodmap_method_constructor = 187 , methodmap_method_destructor = 188 , methodmap_property = 189 , methodmap_property_alias = 190 , methodmap_property_native = 191 , methodmap_property_method = 192 , methodmap_property_getter = 193 , methodmap_property_setter = 194 , r#struct = 195 , struct_field = 196 , struct_declaration = 197 , struct_constructor = 198 , struct_field_value = 199 , r#type = 200 , array_type = 201 , old_type = 202 , dimension = 203 , fixed_dimension = 204 , builtin_type = 205 , old_builtin_type = 206 , block = 207 , for_statement = 208 , while_statement = 209 , do_while_statement = 210 , break_statement = 211 , continue_statement = 212 , condition_statement = 213 , switch_statement = 214 , switch_case = 215 , expression_statement = 216 , return_statement = 217 , delete_statement = 218 , anon__semicolon_ = 219 , anon__expression_ = 220 , anon__case_expression_ = 221 , assignment_expression = 222 , call_expression = 223 , call_arguments = 224 , named_arg = 225 , ignore_argument = 226 , array_indexed_access = 227 , parenthesized_expression = 228 , comma_expression = 229 , ternary_expression = 230 , field_access = 231 , scope_access = 232 , unary_expression = 233 , case_unary_expression = 234 , binary_expression = 235 , case_binary_expression = 236 , update_expression = 237 , anon__sizeof_call_expression_ = 238 , array_scope_access = 239 , sizeof_expression = 240 , view_as = 241 , old_type_cast = 242 , array_literal = 243 , anon__literal_ = 244 , char_literal = 245 , string_literal = 246 , rest_operator = 247 , anon_source_file_repeat1_ = 248 , anon_preproc_macro_repeat1_ = 249 , anon_function_definition_repeat1_ = 250 , anon_parameter_declarations_repeat1_ = 251 , anon_parameter_declaration_repeat1_ = 252 , anon_global_variable_declaration_repeat1_ = 253 , anon_variable_declaration_statement_repeat1_ = 254 , anon_dynamic_array_repeat1_ = 255 , anon_old_global_variable_declaration_repeat1_ = 256 , anon_enum_entries_repeat1_ = 257 , anon_enum_struct_repeat1_ = 258 , anon_typeset_repeat1_ = 259 , anon_funcenum_repeat1_ = 260 , anon_methodmap_repeat1_ = 261 , anon_methodmap_property_repeat1_ = 262 , anon_struct_repeat1_ = 263 , anon_struct_constructor_repeat1_ = 264 , anon_block_repeat1_ = 265 , anon_for_statement_repeat1_ = 266 , anon_switch_statement_repeat1_ = 267 , anon_switch_case_repeat1_ = 268 , anon_call_arguments_repeat1_ = 269 , anon_array_literal_repeat1_ = 270 , anon_string_literal_repeat1_ = 271 } impl From < tree_sitter :: Node < '_ >> for TSKind { fn from (v : tree_sitter :: Node < '_ >) -> Self { unsafe { :: std :: mem :: transmute (v . kind_id ()) } } } impl From < & tree_sitter :: Node < '_ >> for TSKind { fn from (v : & tree_sitter :: Node < '_ >) -> Self { Self :: from (* v) } }
