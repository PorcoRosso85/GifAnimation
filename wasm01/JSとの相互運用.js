WebAssembly.compileStreaming(fetch("sections.wasm"))
.then(mod => {
  const sections = WebAssembly.module.customSections(mod, "hello");
  const decoder = new TextDecoder();
  const text = decoder.decode(sections[0]);
  
  console.log(text);
});



  WebAssembly.compileStreaming(fetch("sections.wasm"))
  .then(mod => {
    const sections = WebAssembly.Module.customSections(mod, "hello");
    const decoder = new TextDecoder();
    const text = decoder.decode(sections[0]);
    
    console.log(text);
  });