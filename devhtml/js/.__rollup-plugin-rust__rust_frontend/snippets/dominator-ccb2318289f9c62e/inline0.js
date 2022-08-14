
(function(l, r) { if (!l || l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (self.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(self.document);
import '../../../_virtual/_rollup-plugin-inject-process-env.js';

function set_property(obj, name, value) { obj[name] = value; }

    function add_event(elem, name, capture, passive, f) {
        elem.addEventListener(name, f, {
            capture,
            passive,
            once: false,
        });
    }

export { add_event, set_property };
//# sourceMappingURL=inline0.js.map
