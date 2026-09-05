// Fase 2 — Entrypoint de la stdlib JavaScript (expone UI y useState como globales).
(function () {
  var c = globalThis.__axoComponents || {};
  var UI = { View: c.View, Text: c.Text, Button: c.Button, Input: c.Input };
  globalThis.UI = UI;
  // Fase 2: useState ya fue asignado por state.js; no se redefine aquí.
})();
