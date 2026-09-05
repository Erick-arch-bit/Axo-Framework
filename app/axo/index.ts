// Fase 3 — Entrypoint de la stdlib (versión TypeScript).
(function (): void {
  const g = globalThis as any;
  const c = g.__axoComponents || {};
  const UI = { View: c.View, Text: c.Text, Button: c.Button, Input: c.Input };
  g.UI = UI;
  // Fase 3: useState ya fue asignado por state.ts; no se redefine aquí.
})();
