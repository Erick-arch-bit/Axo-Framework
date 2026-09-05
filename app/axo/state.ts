// Fase 3 — Estado mínimo (useState) para la stdlib (versión TypeScript).
(function (): void {
  const g = globalThis as any;
  if (!g.__AXO_STATE) {
    g.__AXO_STATE = {};
  }

  function useState(key: string, initialValue: any): [any, (next: any) => void] {
    if (!(key in g.__AXO_STATE)) {
      g.__AXO_STATE[key] = initialValue;
    }
    function setValue(next: any): void {
      g.__AXO_STATE[key] = next;
    }
    return [g.__AXO_STATE[key], setValue];
  }

  g.useState = useState;
})();
