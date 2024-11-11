use itertools::Itertools;
use oxc::{
  ast::ast::{self},
  semantic::SymbolId,
  span::{Atom, SPAN},
};
use rolldown_common::{
  EcmaModuleAstUsage, ExportsKind, LocalExport, Module, ModuleId, ModuleIdx, ModuleType,
  StmtInfoIdx,
};
use rolldown_ecmascript::EcmaAst;
use rolldown_ecmascript_utils::{AstSnippet, TakeIn};
use rolldown_utils::rayon::{IntoParallelRefMutIterator, ParallelIterator};
use rustc_hash::FxHashMap;

use super::LinkStage;

impl<'link> LinkStage<'link> {
  pub fn generate_lazy_export(&mut self) {
    let module_idx_to_exports_kind = append_only_vec::AppendOnlyVec::new();
    self.module_table.modules.par_iter_mut().for_each(|module| {
      let Module::Normal(module) = module else {
        return;
      };
      if !module.meta.has_lazy_export() {
        return;
      }
      let default_symbol_ref = module.default_export_ref;
      module
        .named_exports
        .insert("default".into(), LocalExport { span: SPAN, referenced: default_symbol_ref });

      let is_json = matches!(module.module_type, ModuleType::Json);
      // Json is special, default expr may not be the second stmt;
      if !is_json {
        module.stmt_infos.declare_symbol_for_stmt(1.into(), default_symbol_ref);
      }
      module_idx_to_exports_kind.push((module.ecma_ast_idx(), module.exports_kind, is_json));

      // generate `module.exports = expr`
      if module.exports_kind == ExportsKind::CommonJs {
        // since the wrap arguments are generate on demand, we need to insert the module ref usage here.
        module.stmt_infos.infos[StmtInfoIdx::new(1)].side_effect = true;
        module.ecma_view.ast_usage.insert(EcmaModuleAstUsage::ModuleRef);
      }
    });

    for (ast_idx, exports_kind, is_json_module) in module_idx_to_exports_kind {
      let Some((ecma_ast, module_idx)) = self.ast_table.get_mut(ast_idx) else {
        continue;
      };
      let module_idx = *module_idx;

      if matches!(exports_kind, ExportsKind::CommonJs) {
        ecma_ast.program.with_mut(|fields| {
          let snippet = AstSnippet::new(fields.allocator);
          let Some(stmt) = fields.program.body.first_mut() else { unreachable!() };
          let expr = match stmt {
            ast::Statement::ExpressionStatement(stmt) => stmt.expression.take_in(snippet.alloc()),
            _ => {
              unreachable!()
            }
          };
          *stmt = snippet.module_exports_expr_stmt(expr);
        });
        return;
      }
      // ExportsKind == Esm && ModuleType == Json
      if is_json_module {
        json_object_expr_to_esm(self, module_idx);
        return;
      }
      ecma_ast.program.with_mut(|fields| {
        let snippet = AstSnippet::new(fields.allocator);
        let Some(stmt) = fields.program.body.first_mut() else { unreachable!() };
        let expr = match stmt {
          ast::Statement::ExpressionStatement(stmt) => stmt.expression.take_in(snippet.alloc()),
          _ => {
            unreachable!()
          }
        };
        *stmt = snippet.export_default_expr_stmt(expr);
      });
    }
  }
}

fn json_object_expr_to_esm(link_staged: &mut LinkStage, module_idx: ModuleIdx) {
  let module = &mut link_staged.module_table.modules[module_idx];
  let Module::Normal(module) = module else {
    return;
  };
  // remove all StmtInfo except the 0, which is preserved for namespace binding
  module.stmt_infos.drain(1.into()..);
  dbg!(&module.namespace_object_ref);
  dbg!(&module.default_export_ref);
  // let ecma_ast = link_staged.ast_table.get_mut(module.ecma_ast_idx()).unwrap();
  // let json_like_ast = ecma_ast.json_like_ast().unwrap();
  // let properties = extract_object_properties_from_json_like_ast(json_like_ast).unwrap();
  // let mut snippet = AstSnippet::new(ecma_ast.program().allocator);
  // let mut stmts = Vec::with_capacity(properties.len());
  // for property in properties {
  //   let expr = snippet.json_property_to_esm_export(property);
  //   stmts.push(snippet.export_named_expr_stmt(property, expr));
  // }
  // ecma_ast.program.with_mut(|fields| {
  //   fields.program.body = stmts;
  // });
}

fn extract_object_properties_from_json_like_ast<'a>(
  ast: &'a EcmaAst,
) -> impl Iterator<Item = Atom<'a>> {
  let program = ast.program();
  let Some(first_stmt) = program.body.first() else {
    return itertools::Either::Left(std::iter::empty());
  };
  let expr = match first_stmt {
    ast::Statement::ExpressionStatement(expr) => &expr.expression,
    _ => {
      return itertools::Either::Left(std::iter::empty());
    }
  };

  let ast::Expression::ObjectExpression(obj_expr) = expr.without_parentheses() else {
    return itertools::Either::Left(std::iter::empty());
  };
  let ret = obj_expr.properties.iter().filter_map(|kind| match kind {
    ast::ObjectPropertyKind::ObjectProperty(ref property) => match property.key {
      ast::PropertyKey::StringLiteral(ref key) => Some(key.value.clone()),
      _ => None,
    },
    ast::ObjectPropertyKind::SpreadProperty(_) => None,
  });

  itertools::Either::Right(ret)
}
