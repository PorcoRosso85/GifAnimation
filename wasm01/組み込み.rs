// main.rs

fn js_hello(ctx: &mut Context) {
  let code = r#"print('hello quickjs')"#;
  let r = ctx.eval_global_str(code);
  println!("return value:{:?}", r);
}