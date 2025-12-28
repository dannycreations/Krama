use std::sync::Arc;

use ahash::AHashMap;
use krama_core::{
  Binding, Enum, EnumInstance, EnumVariant, Expression, Function, FunctionBody,
  Object, ObjectResult, Parameter, Span, Struct, StructField, StructMethod,
  Type,
};

use crate::interpreter::{types::resolve_type, Interpreter};

impl Interpreter {
  pub async fn eval_let_statement(
    &self,
    binding: &Binding,
    value_expr: &Expression,
    ty: Option<&Type>,
  ) -> ObjectResult {
    let mut value = self.eval_and_check_type(value_expr, ty).await?;
    value.set_constant(false);
    self.apply_binding(binding, value, false, value_expr.span, false)?;
    Ok(Object::Void)
  }

  pub async fn eval_const_statement(
    &self,
    binding: &Binding,
    value_expr: &Expression,
    public: bool,
    ty: Option<&Type>,
    span: Span,
  ) -> ObjectResult {
    let mut value = self.eval_and_check_type(value_expr, ty).await?;
    value.set_constant(true);
    self.apply_binding(binding, value, public, span, true)?;
    Ok(Object::Void)
  }

  pub async fn eval_function_statement(
    &self,
    name: &Arc<str>,
    parameters: &[Parameter],
    body: &FunctionBody,
    public: bool,
    ty: Option<&Type>,
  ) -> ObjectResult {
    let resolved_ty = ty.map(|k| resolve_type(self, k)).transpose()?;
    let function =
      self.alloc_user_function(parameters.to_vec(), body.clone(), resolved_ty);
    self
      .stack
      .write()
      .define(name.clone(), function, public, true);
    Ok(Object::Void)
  }

  pub async fn eval_enum_statement(
    &self,
    name: &Arc<str>,
    variants: &[EnumVariant],
    public: bool,
  ) -> ObjectResult {
    let name_arc = name.clone();
    let mut properties = AHashMap::with_capacity(variants.len());
    for variant in variants {
      let variant_name_arc = variant.name.clone();
      let obj = if let Some(fields) = &variant.fields {
        Object::Function(Function::Enum(Arc::new(Enum {
          name: name_arc.clone(),
          variant: variant_name_arc.clone(),
          field_count: fields.len(),
        })))
      } else {
        Object::Enum(Box::new(EnumInstance {
          name: name_arc.clone(),
          variant: variant_name_arc.clone(),
          fields: None,
        }))
      };
      properties.insert(variant_name_arc, obj);
    }
    let enum_obj = self.heap.write().alloc_object(
      properties.into_iter().collect(),
      None,
      true,
    );
    self
      .stack
      .write()
      .define(name.clone(), enum_obj, public, true);
    Ok(Object::Void)
  }

  pub async fn eval_struct_statement(
    &self,
    name: &Arc<str>,
    fields: &[StructField],
    methods: &[StructMethod],
    public: bool,
  ) -> ObjectResult {
    let name_arc = name.clone();
    let field_map = fields
      .iter()
      .enumerate()
      .map(|(i, f)| (f.name.clone(), i))
      .collect();

    let method_map = methods
      .iter()
      .enumerate()
      .map(|(i, m)| (m.name.clone(), i))
      .collect();

    let struct_def = Arc::new(Struct {
      name: name_arc,
      fields: fields.to_vec(),
      methods: methods.to_vec(),
      field_map,
      method_map,
    });
    self.stack.write().define(
      name.clone(),
      Object::Struct(struct_def),
      public,
      true,
    );
    Ok(Object::Void)
  }

  pub async fn eval_type_statement(
    &self,
    name: &Arc<str>,
    ty: &Type,
    public: bool,
  ) -> ObjectResult {
    let resolved = resolve_type(self, ty)?;
    self.stack.write().define(
      name.clone(),
      Object::Type(resolved),
      public,
      true,
    );
    Ok(Object::Void)
  }
}
