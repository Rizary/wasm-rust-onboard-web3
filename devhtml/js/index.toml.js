
(function(l, r) { if (!l || l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (self.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(self.document);
import './_virtual/_rollup-plugin-inject-process-env.js';
import init from './.__rollup-plugin-rust__rust_frontend/index.js';

init("/js/assets/rust_frontend.wasm").catch(console.error);
//# sourceMappingURL=index.toml.js.map
