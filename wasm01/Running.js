// hello.js
args = args.slice(1)
print("Hello", ...args)


//module_def.js
function hello(){
  console.log('hello from module_def.js')
}
export {hello}


//module_def_async.js
export async function hello() {
  console.log('hello from module_async.js')
  return "module_def_async.js: return value"
}
export var something = "async thing"


//demo.js
import { hello as module_def_hello } from './module_def.js'
module_def_hello()

var f = async () => {
  let {hello, something} = await import('./module_def_async.js')
  await hello()
  console.log("./module_def_async.js `something` is ", something)
}
f()


//http_demo.js, javascript networking client example
let r = GET("http://18.235.124.214/get?a=123", {"a":"b","c":[1,2,3]})
print(r.status)

let headers = r.headers
print(JSON.stringify(headers))let body = r.body;
let body_str = new Uint8Array(body)
print(String.fromCharCode.apply(null, body_str))


//http_server_demo.js, javascript networking server example
import {HttpSever} from 'http'
let http_server = new HttpServer('0.0.0.0:8000')
print('listen on 0.0.0.0:8000')

while(true){
  http_server.accept((request) => {
    let body = request.body
    let body_str = String.fromCharCode.apply(null, new Uint8Array(body))
    print(JSON.stringify(request), '\n body_str:', body_str)
    return {
      status: 200,
      header: {'Content-Type':'application/json'},
      body: 'echo:' + body_str
    }
  });
}


//main.js, tensorflow
import {TensorflowLiteSession} from 'tensorflow_lite'
import {Image} from 'image'let img = new Image('./example_js/tensorflow_lite_demo/food.jpg')

let img_rgb = img.to_rgb().resize(192,192)
let rgb_pix = img_rgb.pixels()let session = new TensorflowLiteSession('./example_js/tensorflow_list_demo/lite-model_aiy_vision_classifier_food_V1_1.tflite')

session.add_input('input', rgb_pix)
session.run()
let output = session.get_output('MobilenetV1/Predictions/Softmax');
let output_view = new Uint8Array(output)
let max = 0;
let max_idx = 0;
for (var i in output_view) {
  let v = output_view[i]
  if( v > max ) {
    max = v;
    max_idx = i;
  }
}
print(max, max_idx)




// hello.js
args = args.slice(1)
print("Hello", ...args)