// Fase 2 — Componentes UI mínimos (View, Text, Button, Input).
(function () {
  if (!globalThis.__AXO_CALLBACKS) {
    globalThis.__AXO_CALLBACKS = {};
  }
  var __axoCbCounter = 0;

  // Fase 2: función → id string guardado en __AXO_CALLBACKS; string → tal cual.
  function __axoNormalizeOnClick(onClick) {
    if (typeof onClick === "function") {
      __axoCbCounter += 1;
      var id = "cb_" + __axoCbCounter;
      globalThis.__AXO_CALLBACKS[id] = onClick;
      return id;
    }
    if (typeof onClick === "string") {
      return onClick;
    }
    return null;
  }

  function View(props) {
    props = props || {};
    return {
      type: "View",
      content: "",
      style: props.style || {},
      children: props.children || []
    };
  }

  function Text(contentOrProps, maybeProps) {
    var content = "";
    var style = {};
    if (typeof contentOrProps === "string" || typeof contentOrProps === "number") {
      content = String(contentOrProps);
      var p = maybeProps || {};
      style = p.style || {};
      if (p.fontSize !== undefined && style.fontSize === undefined) {
        style.fontSize = p.fontSize;
      }
      if (p.color !== undefined && style.color === undefined) {
        style.color = p.color;
      }
    } else {
      var q = contentOrProps || {};
      content = q.content || q.text || "";
      if (typeof content !== "string") {
        content = String(content);
      }
      style = q.style || {};
    }
    return { type: "Text", content: content, style: style, children: [] };
  }

  function Button(props) {
    props = props || {};
    var content = props.text || props.content || "";
    if (typeof content !== "string") {
      content = String(content);
    }
    var node = { type: "Button", content: content, style: props.style || {}, children: [] };
    var oc = __axoNormalizeOnClick(props.onClick);
    if (oc !== null) {
      node.onClick = oc;
    }
    return node;
  }

  function Input(props) {
    props = props || {};
    var content = props.value || props.text || props.content || "";
    if (typeof content !== "string") {
      content = String(content);
    }
    var node = { type: "Input", content: content, style: props.style || {}, children: [] };
    var oc = __axoNormalizeOnClick(props.onClick);
    if (oc !== null) {
      node.onClick = oc;
    }
    return node;
  }

  globalThis.__axoComponents = { View: View, Text: Text, Button: Button, Input: Input };
})();
