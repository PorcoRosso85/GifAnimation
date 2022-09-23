fn run_rust_function(ctx: &mut Context) {
  struct HelloFn;
  impl JsFn for HelloFn {
    fn call (_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
      println!("hello from rust");
      println!("argv={:?}", argv);
      JsValue::UnDefined
    }
  }
  
}



// add hi() function to javascript interpreter, named as "JavaScript API"
fn run_rust_function(ctx: &mut Context) {
  struct HelloFn;
  impl JsFn for HelloFn {
    fn call (_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
      println!("hello from rust");
      println!("argv={:?}", argv);
      JsValue::UnDefined
    }
  }
  let f = ctx.new_function::<HelloFn>("hello");
  ctx.get_global().set("hi", f.into());
  let code = r#"
    hi(1, 2, 3)
  "#;
  let r = ctx.eval_global_str(code);
  println!("return value:{:?}", r);
}



// requirement of object which make both data and function capsule
fn rust_new_object_and_js_call(ctx: &mut Context) {
  // JSAPI side
  struct ObjectFn;
  impl JsFn for ObjectFn {
    fn call(_ctx: &mut Context, this_val: JsValue, argv: &[JsValue]) -> JsValue {
      println!("hello from rust");
      pringln!("argv = {:?}", argv);
      if let JsValue::Object(obj) = this_val {
        let obj_map = obj.to_map();
        println!("this = {:#?}", obj_map);
      }
      JsValue::Undefined
    }
  }
  // Rust side, create object
  let mut obj = ctx.new_object();
  obj.set("a", 1.into());
  obj.set("b", ctx.new_string("abc").into());
  
  let f = ctx.new_function::<ObjectFn>("anything");
  obj.set("f", f.into());
  // Rust side, make object useful
  ctx.get_global().set("text_obj", obj.into());
  
  let code = r#"
    print
    print(
    print(
    text_obj.f(1, 2, 3, "hi")
  "#;
  
  ctx.eval_global_str(code);
}



// complete rust module for use JavaScript Object API
mod point {
  use wasmedge_quickjs::*;
  
  #[derive(Debug)]
  struct Point(i32, i32);
  struct PointDef;
  
  impl JsClassDef<Point> for PointDef {
    const CLASS_NAME: &'static str = "Point\0";
    const CONSTRUCTOR_ARGC: u8 = 2
    
    fn constructor(_: &mut Context, argv: &[JsValue]) -> Option<Point> {
      println!("rust -> new Point {:?}", argv);
      let x = argv.get(0);
      let y = argv.get(1);
      if let ((Some(JsValue::Int(ref x)), Some(JsValue::Int(ref y)))) = (x, y) {
        Some(Point(*x, *y))
      } else {
        None
      }
    }
    
    