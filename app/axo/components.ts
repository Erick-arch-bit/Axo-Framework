// Fase 3 — Componentes UI mínimos (versión TypeScript).
interface AxoStyle {
  [key: string]: any;
}

interface AxoNode {
  type: string;
  content: string;
  style: AxoStyle;
  children: AxoNode[];
  onClick?: string;
}

interface ViewProps {
  style?: AxoStyle;
  children?: AxoNode[];
}

interface TextProps {
  content?: any;
  text?: any;
  style?: AxoStyle;
  fontSize?: number;
  color?: string;
}

interface ButtonProps {
  text?: any;
  content?: any;
  style?: AxoStyle;
  onClick?: ((...args: any[]) => void) | string;
}

(function (): void {
  const g = globalThis as any;
  if (!g.__AXO_CALLBACKS) {
    g.__AXO_CALLBACKS = {};
  }
  let __axoCbCounter: number = 0;

  // Fase 3: función → id string guardado en __AXO_CALLBACKS; string → tal cual.
  function __axoNormalizeOnClick(onClick: any): string | null {
    if (typeof onClick === "function") {
      __axoCbCounter += 1;
      const id: string = "cb_" + __axoCbCounter;
      g.__AXO_CALLBACKS[id] = onClick;
      return id;
    }
    if (typeof onClick === "string") {
      return onClick;
    }
    return null;
  }

  function View(props: ViewProps = {}): AxoNode {
    return {
      type: "View",
      content: "",
      style: props.style || {},
      children: props.children || []
    };
  }

  function Text(contentOrProps: any, maybeProps: TextProps = {}): AxoNode {
    let content: string = "";
    let style: AxoStyle = {};
    if (typeof contentOrProps === "string" || typeof contentOrProps === "number") {
      content = String(contentOrProps);
      const p: TextProps = maybeProps || {};
      style = p.style || {};
      if (p.fontSize !== undefined && style.fontSize === undefined) {
        style.fontSize = p.fontSize;
      }
      if (p.color !== undefined && style.color === undefined) {
        style.color = p.color;
      }
    } else {
      const q: TextProps = contentOrProps || {};
      content = q.content || q.text || "";
      if (typeof content !== "string") {
        content = String(content);
      }
      style = q.style || {};
    }
    return { type: "Text", content: content, style: style, children: [] };
  }

  function Button(props: ButtonProps = {}): AxoNode {
    let content: string = (props.text || props.content || "") as string;
    if (typeof content !== "string") {
      content = String(content);
    }
    const node: AxoNode = { type: "Button", content: content, style: props.style || {}, children: [] };
    const oc: string | null = __axoNormalizeOnClick(props.onClick);
    if (oc !== null) {
      node.onClick = oc;
    }
    return node;
  }

  function Input(props: ButtonProps = {}): AxoNode {
    const raw: any = (props as any).value || props.text || props.content || "";
    let content: string = raw as string;
    if (typeof content !== "string") {
      content = String(content);
    }
    const node: AxoNode = { type: "Input", content: content, style: props.style || {}, children: [] };
    const oc: string | null = __axoNormalizeOnClick(props.onClick);
    if (oc !== null) {
      node.onClick = oc;
    }
    return node;
  }

  g.__axoComponents = { View: View, Text: Text, Button: Button, Input: Input };
})();
