// Fase 2 — Estado mínimo (useState) para la stdlib JavaScript.
(function () {
  if (!globalThis.__AXO_STATE) {
    globalThis.__AXO_STATE = {};
  }

  function useState(key, initialValue) {
    if (!(key in globalThis.__AXO_STATE)) {
      globalThis.__AXO_STATE[key] = initialValue;
    }
    function setValue(next) {
      globalThis.__AXO_STATE[key] = next;
    }
    return [globalThis.__AXO_STATE[key], setValue];
  }

  globalThis.useState = useState;
})();
