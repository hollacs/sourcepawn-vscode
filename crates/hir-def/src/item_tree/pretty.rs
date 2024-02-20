use la_arena::Idx;

use crate::db::DefDatabase;

use super::{
    Enum, EnumStruct, EnumStructItemId, Field, FileItem, Function, ItemTree, Macro,
    RawVisibilityId, Variable, Variant,
};

pub fn print_item_tree(_db: &dyn DefDatabase, tree: &ItemTree) -> String {
    let mut printer = Printer::new(tree);
    for item in tree.top_level_items() {
        match item {
            FileItem::Function(idx) => printer.print_function(idx),
            FileItem::Variable(idx) => {
                let Variable { name, .. } = &tree[*idx];
                printer.push(name.0.as_str());
                printer.push("\n");
            }
            FileItem::EnumStruct(idx) => printer.print_enum_struct(idx),
            FileItem::Enum(idx) => printer.print_enum(idx),
            FileItem::Macro(idx) => printer.print_macro(idx),
            FileItem::Variant(_) => (),
        }
    }

    printer.into_string()
}

struct Printer<'a> {
    buf: String,
    indent_level: usize,
    tree: &'a ItemTree,
}

impl<'a> Printer<'a> {
    fn new(tree: &'a ItemTree) -> Self {
        Self {
            buf: String::new(),
            indent_level: 0,
            tree,
        }
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        self.indent_level -= 1;
    }

    fn newline(&mut self) {
        self.buf.push('\n');
        self.buf.push_str("  ".repeat(self.indent_level).as_str());
    }

    fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn into_string(self) -> String {
        self.buf
    }

    pub fn print_macro(&mut self, idx: &Idx<Macro>) {
        let Macro { name, ast_id, .. } = &self.tree[*idx];
        self.push(format!("// {}", ast_id).as_str());
        self.newline();
        self.push(&format!("#define {}", name.0.to_string()));
        self.newline();
    }

    pub fn print_enum(&mut self, idx: &Idx<Enum>) {
        let Enum {
            name,
            variants,
            ast_id,
            ..
        } = &self.tree[*idx];
        self.push(format!("// {}", ast_id).as_str());
        self.newline();
        self.push(&format!("enum {} {{", name.0));
        self.indent();
        self.newline();
        for variant in self.tree.data().variants[variants.clone()].iter() {
            let Variant { name, ast_id, .. } = variant;
            self.push(format!("// {}", ast_id).as_str());
            self.newline();
            self.push(&format!("{},", name.0));
            self.newline();
        }
        self.dedent();
        self.push("};");
        self.newline();
    }

    pub fn print_enum_struct(&mut self, idx: &Idx<EnumStruct>) {
        let EnumStruct {
            name,
            items,
            ast_id,
            ..
        } = &self.tree[*idx];
        self.push(format!("// {}", ast_id).as_str());
        self.newline();
        self.push(&format!("{} {{", name.0));
        self.indent();
        self.newline();
        for item_idx in items.iter() {
            match item_idx {
                EnumStructItemId::Field(field_idx) => {
                    let Field { name, type_ref, .. } = &self.tree[*field_idx];
                    self.push(format!("// {}", ast_id).as_str());
                    self.newline();
                    self.push(&format!("{} {};", type_ref.to_str(), name.0));
                    self.newline();
                }
                EnumStructItemId::Method(method_idx) => self.print_function(method_idx),
            }
        }
        self.dedent();
        self.newline();
        self.push("}");
        self.newline();
    }

    pub fn print_function(&mut self, idx: &Idx<Function>) {
        let Function {
            name,
            visibility,
            ret_type,
            params,
            ast_id,
            ..
        } = &self.tree[*idx];
        self.push(format!("// {}", ast_id).as_str());
        self.newline();
        if visibility != &RawVisibilityId::NONE {
            self.push(&visibility.to_string());
            self.push(" ");
        }
        if let Some(ret_type) = ret_type {
            self.push(&ret_type.to_str());
            self.push(" ");
        }
        self.push(&name.0);
        self.push("(");
        self.indent();
        for param in self.tree.data().params[params.clone()].iter() {
            if let Some(type_ref) = &param.type_ref {
                self.newline();
                self.push(format!("// {}", param.ast_id).as_str());
                self.newline();
                self.push(&type_ref.to_str());
                self.push(",");
            }
        }
        self.push(") {");
        self.newline();
        self.push("/* body */");
        self.dedent();
        self.newline();
        self.push("}");
        self.newline();
    }
}
