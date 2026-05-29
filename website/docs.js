var DOCS_DATA = [
    {
        label: "Empezar",
        pages: [
            { id: "intro", title: "Introducción" },
            { id: "install", title: "Instalación" },
            { id: "quickstart", title: "Quickstart" },
            { id: "project-structure", title: "Estructura del proyecto" },
        ],
    },
    {
        label: "Guías",
        pages: [
            { id: "cli", title: "CLI Reference" },
            { id: "components", title: "Componentes" },
            { id: "layout", title: "Layout y Estilos" },
            { id: "events", title: "Eventos" },
            { id: "device-api", title: "Device API" },
            { id: "permissions", title: "Permisos" },
            { id: "hot-reload", title: "Hot Reload" },
        ],
    },
    {
        label: "Referencia",
        pages: [
            { id: "style-props", title: "Propiedades de estilo" },
            { id: "architecture", title: "Arquitectura" },
            { id: "adrs", title: "ADRs" },
        ],
    },
];

var DOCS_PAGES = {
    "intro": {
        title: "Introducción a Axo",
        description: "¿Qué es Axo y por qué usarlo?",
        content: `
            <p><strong>Axo</strong> es un framework cross-platform para construir aplicaciones nativas rápidas con <strong>Rust</strong> en el motor y <strong>Lua</strong> en la UI.</p>

            <div class="highlight">
                <p><strong>⚡ Filosofía:</strong> <em>"Crea más rápido. Hazlo completo. Extiéndelo todo."</em></p>
            </div>

            <p>Axo está diseñado para aplicaciones de nicho — dashboards embebidos, terminales POS, herramientas internas — donde Flutter o Electron son demasiado pesados y las web apps no tienen suficiente acceso al hardware.</p>

            <h2>¿Por qué Axo?</h2>
            <ul>
                <li><strong>Rust nativo</strong> — Renderizado con wgpu (Vulkan, Metal, DX12, WebGL2). Sin runtime pesado.</li>
                <li><strong>UI en Lua</strong> — Interfaz declarativa desde Lua 5.4. Sin JS, sin npm, sin node_modules.</li>
                <li><strong>Layout Flexbox</strong> — Taffy engine. CSS Flexbox completo con posicionamiento absoluto.</li>
                <li><strong>Hot reload</strong> — Edita tu código Lua y ve los cambios al instante.</li>
                <li><strong>Permisos granulares</strong> — Sistema de permisos con default Denied.</li>
                <li><strong>Multiplataforma</strong> — Linux, macOS, Windows, Android, iOS, Web.</li>
            </ul>

            <h2>Stack técnico</h2>
            <table>
                <tr><th>Capa</th><th>Tecnología</th></tr>
                <tr><td>Core engine</td><td>Rust</td></tr>
                <tr><td>Scripting UI</td><td>Lua 5.4 (mlua)</td></tr>
                <tr><td>Renderizado</td><td>wgpu (Vulkan/Metal/DX12/WebGL2)</td></tr>
                <tr><td>Layout</td><td>Taffy (Flexbox/CSS Grid)</td></tr>
                <tr><td>Ventanas</td><td>winit</td></tr>
                <tr><td>Hot reload</td><td>notify (file watcher)</td></tr>
                <tr><td>Texto</td><td>ab_glyph + LiberationSans</td></tr>
            </table>

            <h2>Targets</h2>
            <p>Axo apunta a aplicaciones de nicho donde otras soluciones sobran:</p>
            <ul>
                <li>Dashboards industriales embebidos</li>
                <li>Terminales POS y kioskos</li>
                <li>Apps internas de empresas</li>
                <li>Herramientas de diagnóstico</li>
                <li>Prototipos rápidos multiplataforma</li>
            </ul>
        `,
    },
    "install": {
        title: "Instalación",
        description: "Instala Axo en tu sistema.",
        content: `
            <h2>Prerequisitos</h2>
            <ul>
                <li>Rust toolchain (rustc &ge; 1.75, cargo)</li>
                <li>Sistema operativo: Linux, macOS o Windows</li>
            </ul>

            <h2>Via cargo (recomendado)</h2>
            <pre><code>cargo install axo-cli</code></pre>

            <p>Esto instala el binario <code>axo-cli</code> en tu PATH.</p>

            <h2>Verificar instalación</h2>
            <pre><code>axo-cli --version</code></pre>

            <p>Deberías ver algo como:</p>
            <pre><code>axo-cli 0.1.3</code></pre>

            <h2>Desde fuente</h2>
            <pre><code>git clone https://github.com/Erick-arch-bit/Axo-Framework.git
cd Axo-Framework
cargo build --release
./target/release/axo-cli --version</code></pre>

            <h2>Dependencias del sistema</h2>
            <p>En Linux,可能需要 instalar algunas dependencias de wgpu:</p>
            <pre><code># Ubuntu/Debian
sudo apt install libx11-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libxkbcommon-dev libwayland-dev

# Fedora
sudo dnf install libX11-devel libxcb-devel libxkbcommon-devel wayland-devel</code></pre>
        `,
    },
    "quickstart": {
        title: "Quickstart",
        description: "Crea tu primera app en 2 minutos.",
        content: `
            <h2>1. Scaffold un proyecto</h2>
            <pre><code>axo-cli init mi-app
cd mi-app</code></pre>

            <p>Esto crea la siguiente estructura:</p>
            <pre><code>mi-app/
├── app/
│   ├── axo/
│   │   └── init.lua      # Std library
│   └── app.lua           # Entry point
└── README.md</code></pre>

            <h2>2. Inicia el dev server</h2>
            <pre><code>axo-cli dev</code></pre>

            <p>Esto abre una ventana con tu app y activa hot reload.</p>

            <h2>3. Edita tu app</h2>
            <p>Abre <code>app/app.lua</code> y cámbialo:</p>
            <pre><code><span class="kw">local</span> UI = <span class="kw">require</span>(<span class="s">"axo"</span>)

<span class="kw">function</span> <span class="nf">onClick</span>()
    <span class="nb">log</span>(<span class="s">"Hello from Axo!"</span>)
<span class="kw">end</span>

<span class="kw">function</span> <span class="nf">App</span>()
    <span class="kw">return</span> UI.View({
        style = {
            width = <span class="s">"100%"</span>,
            height = <span class="s">"100%"</span>,
            backgroundColor = <span class="s">"#1a1a2e"</span>,
            flexDirection = <span class="s">"column"</span>,
            justifyContent = <span class="s">"center"</span>,
            alignItems = <span class="s">"center"</span>,
        },
        children = {
            UI.Text({
                text = <span class="s">"Hola desde Axo!"</span>,
                style = { fontSize = <span class="n">28</span>, color = <span class="s">"#ffffff"</span> },
            }),
            UI.Button({
                text = <span class="s">"Clic aquí"</span>,
                onClick = <span class="s">"onClick"</span>,
                style = {
                    backgroundColor = <span class="s">"#e94560"</span>,
                    width = <span class="s">"200"</span>,
                    height = <span class="s">"50"</span>,
                    margin = <span class="s">"10"</span>,
                },
            }),
        },
    })
<span class="kw">end</span>

<span class="kw">return</span> App</code></pre>

            <h2>4. Build para producción</h2>
            <pre><code>axo-cli build --release</code></pre>
        `,
    },
    "project-structure": {
        title: "Estructura del proyecto",
        description: "Cómo se organiza un proyecto Axo.",
        content: `
            <p>Un proyecto Axo generado con <code>axo-cli init</code> tiene la siguiente estructura:</p>

            <pre><code>mi-app/
├── app/
│   ├── axo/
│   │   └── init.lua      # Std library (componentes + device wrappers)
│   └── app.lua           # Entry point: debe retornar una función App()
├── README.md</code></pre>

            <h2>app/app.lua</h2>
            <p>Archivo principal de tu aplicación. Debe retornar una función <code>App()</code> que devuelva un árbol de UI:</p>

            <pre><code><span class="kw">local</span> UI = <span class="kw">require</span>(<span class="s">"axo"</span>)

<span class="kw">function</span> <span class="nf">App</span>()
    <span class="kw">return</span> UI.View({ ... })
<span class="kw">end</span>

<span class="kw">return</span> App</code></pre>

            <h2>app/axo/init.lua</h2>
            <p>Librería estándar de Axo. Contiene todos los componentes (<code>View</code>, <code>Text</code>, <code>Button</code>, etc.) y wrappers de Device API. Se puede modificar para extender o cambiar comportamientos globales.</p>

            <h2>Package path</h2>
            <p>El loader de Lua busca módulos en:</p>
            <ol>
                <li><code>app/axo/?.lua</code> — para <code>require("axo")</code></li>
                <li><code>app/?/init.lua</code> — para módulos en carpetas</li>
                <li><code>app/?.lua</code> — para módulos directos</li>
            </ol>
        `,
    },
    "cli": {
        title: "CLI Reference",
        description: "Todos los comandos de axo-cli.",
        content: `
            <p><code>axo-cli</code> es la herramienta de línea de comandos para desarrollar y compilar apps Axo.</p>

            <h2>Comandos</h2>
            <table>
                <tr><th>Comando</th><th>Descripción</th></tr>
                <tr><td><code>init &lt;name&gt;</code></td><td>Scaffold un nuevo proyecto</td></tr>
                <tr><td><code>dev</code></td><td>Inicia dev server con hot reload</td></tr>
                <tr><td><code>build</code></td><td>Compila la app (usa cargo build internamente)</td></tr>
                <tr><td><code>release</code></td><td>Build + bundle para distribución</td></tr>
            </table>

            <h2>init</h2>
            <pre><code>axo-cli init [name]

# Por defecto crea "my-axo-app"
axo-cli init

# O especifica un nombre
axo-cli init mi-dashboard</code></pre>

            <h2>dev</h2>
            <pre><code>axo-cli dev</code></pre>
            <p>Inicia el modo desarrollo: compila el engine Rust si es necesario, abre una ventana, y monitorea cambios en <code>app/</code> para hot reload.</p>

            <h2>build</h2>
            <pre><code>axo-cli build [--release]</code></pre>
            <p>Compila el engine y empaqueta la app. Usa <code>--release</code> para optimizar.</p>

            <h2>release</h2>
            <pre><code>axo-cli release</code></pre>
            <p>Compila en release y prepara los artefactos para distribución.</p>
        `,
    },
    "components": {
        title: "Componentes",
        description: "Referencia de todos los componentes UI.",
        content: `
            <p>Axo incluye una librería estándar de componentes declarativos. Todos los componentes reciben una tabla <code>props</code> con estilo y callbacks.</p>

            <h2>View</h2>
            <p>Contenedor base. Soporta children, backgroundColor, flexbox.</p>
            <pre><code>UI.View({
    style = { ... },
    children = { ... },
    onClick = "handlerName",   -- opcional
})</code></pre>

            <h2>Text</h2>
            <p>Texto renderizado con ab_glyph. Usa <code>props.text</code> para el contenido.</p>
            <pre><code>UI.Text({
    text = "Hello",
    style = { fontSize = 16, color = "#ffffff" },
    onClick = "handlerName",
})</code></pre>

            <div class="highlight">
                <p><strong>Nota:</strong> Text usa <code>props.text</code> como tabla unificada (no dos argumentos posicionales). Consistente con el resto de componentes.</p>
            </div>

            <h2>Button</h2>
            <p>Botón clickeable con texto.</p>
            <pre><code>UI.Button({
    text = "Enviar",
    onClick = "handleSubmit",
    style = {
        backgroundColor = "#e94560",
        width = "200",
        height = "50",
    },
})</code></pre>

            <h2>Image</h2>
            <p>Muestra una imagen desde source (actualmente placeholder).</p>
            <pre><code>UI.Image({
    source = "path/to/image.png",
    style = { width = "100", height = "100" },
})</code></pre>

            <h2>TextInput</h2>
            <p>Campo de texto con soporte para onChangeText.</p>
            <pre><code>UI.TextInput({
    text = "valor inicial",
    onChangeText = "handleTextChange",
    style = { ... },
})</code></pre>

            <h2>ScrollView</h2>
            <p>Contenedor con scroll (vertical por defecto).</p>
            <pre><code>UI.ScrollView({
    style = { ... },
    children = { ... },
})</code></pre>

            <h2>FlatList</h2>
            <p>Lista eficiente con <code>data</code> + <code>renderItem</code>.</p>
            <pre><code>UI.FlatList({
    data = { "a", "b", "c" },
    renderItem = function(ctx)
        return UI.Text({ text = ctx.item })
    end,
    style = { ... },
})</code></pre>

            <h2>Spacer</h2>
            <p>Espaciador flexible. Usa <code>size</code>, <code>horizontal</code>, <code>flex</code>.</p>
            <pre><code>UI.Spacer({ size = "10" })              -- vertical
UI.Spacer({ horizontal = true })         -- horizontal
UI.Spacer({ flex = true })               -- flex grow</code></pre>

            <h2>Divider</h2>
            <p>Línea divisoria.</p>
            <pre><code>UI.Divider({ color = "#333333", margin = "4" })
UI.Divider({ vertical = true })</code></pre>

            <h2>SafeAreaView</h2>
            <p>View con padding seguro para áreas de notch.</p>
            <pre><code>UI.SafeAreaView({
    style = { ... },
    children = { ... },
})</code></pre>

            <h2>Pressable</h2>
            <p>Wrapper clickeable. Acepta <code>onPress</code> o <code>onClick</code>.</p>
            <pre><code>UI.Pressable({
    onPress = "handlePress",
    style = { ... },
    children = { ... },
})</code></pre>
        `,
    },
    "layout": {
        title: "Layout y Estilos",
        description: "Sistema de layout Flexbox con Taffy.",
        content: `
            <p>Axo usa <strong>Taffy</strong> para layout Flexbox. Todas las propiedades de estilo se definen en la tabla <code>style</code> de cada componente.</p>

            <h2>Unidades</h2>
            <table>
                <tr><th>Formato</th><th>Ejemplo</th><th>Descripción</th></tr>
                <tr><td>Número</td><td><code>200</code></td><td>Píxeles</td></tr>
                <tr><td>String numérico</td><td><code>"200"</code></td><td>Píxeles</td></tr>
                <tr><td>Porcentaje</td><td><code>"50%"</code></td><td>Porcentaje del contenedor</td></tr>
                <tr><td>Auto</td><td><code>"auto"</code></td><td>Automático (margin)</td></tr>
            </table>

            <h2>Propiedades de estilo</h2>
            <table>
                <tr><th>Propiedad</th><th>Tipo</th><th>Descripción</th></tr>
                <tr><td><code>width</code></td><td>length/%</td><td>Ancho</td></tr>
                <tr><td><code>height</code></td><td>length/%</td><td>Alto</td></tr>
                <tr><td><code>minWidth</code></td><td>length/%</td><td>Ancho mínimo</td></tr>
                <tr><td><code>maxWidth</code></td><td>length/%</td><td>Ancho máximo</td></tr>
                <tr><td><code>minHeight</code></td><td>length/%</td><td>Alto mínimo</td></tr>
                <tr><td><code>maxHeight</code></td><td>length/%</td><td>Alto máximo</td></tr>
                <tr><td><code>backgroundColor</code></td><td>hex</td><td>Color de fondo (#rrggbb o #rrggbbaa)</td></tr>
                <tr><td><code>margin</code></td><td>rect/num</td><td>Margen (ej: "10" o "10 5 10 5")</td></tr>
                <tr><td><code>padding</code></td><td>rect/num</td><td>Padding interno</td></tr>
                <tr><td><code>flexDirection</code></td><td>string</td><td>"row", "column", "rowReverse", "columnReverse"</td></tr>
                <tr><td><code>justifyContent</code></td><td>string</td><td>"flexStart", "center", "flexEnd", "spaceBetween", etc.</td></tr>
                <tr><td><code>alignItems</code></td><td>string</td><td>"flexStart", "center", "flexEnd", "stretch", "baseline"</td></tr>
                <tr><td><code>alignSelf</code></td><td>string</td><td>Sobrescribe alignItems para este elemento</td></tr>
                <tr><td><code>flexWrap</code></td><td>string</td><td>"nowrap", "wrap", "wrapReverse"</td></tr>
                <tr><td><code>flexGrow</code></td><td>num</td><td>Factor de crecimiento (0 por defecto)</td></tr>
                <tr><td><code>flexShrink</code></td><td>num</td><td>Factor de encogimiento (1 por defecto)</td></tr>
                <tr><td><code>gap</code></td><td>num</td><td>Espacio entre hijos</td></tr>
                <tr><td><code>position</code></td><td>string</td><td>"relative" o "absolute"</td></tr>
                <tr><td><code>top</code>/<code>left</code>/<code>right</code>/<code>bottom</code></td><td>length/%</td><td>Posicionamiento absoluto</td></tr>
                <tr><td><code>fontSize</code></td><td>num</td><td>Tamaño de fuente (para Text)</td></tr>
                <tr><td><code>color</code></td><td>hex</td><td>Color de texto</td></tr>
                <tr><td><code>borderWidth</code></td><td>num</td><td>Ancho de borde</td></tr>
                <tr><td><code>borderColor</code></td><td>hex</td><td>Color de borde</td></tr>
            </table>

            <h2>Colores</h2>
            <p>Los colores se especifican en formato hexadecimal:</p>
            <pre><code>"#ffffff"    -- blanco
"#ff0000"    -- rojo
"#1a1a2e"    -- azul oscuro
"#e94560"    -- rojo coral</code></pre>
        `,
    },
    "events": {
        title: "Eventos",
        description: "Sistema de eventos onClick y callbacks.",
        content: `
            <p>Axo maneja eventos a través de callbacks definidos en Lua. Actualmente el evento principal es <strong>onClick</strong>.</p>

            <h2>onClick</h2>
            <p>Se puede pasar como nombre de función global (string) o como función anónima:</p>

            <pre><code><span class="c">-- Como string (recomendado para hot reload)</span>
UI.Button({
    text = <span class="s">"Clic"</span>,
    onClick = <span class="s">"handleClick"</span>,
})

<span class="kw">function</span> <span class="nf">handleClick</span>()
    <span class="nb">log</span>(<span class="s">"Clicked!"</span>)
<span class="kw">end</span></code></pre>

            <pre><code><span class="c">-- Como función anónima</span>
UI.Button({
    text = <span class="s">"Clic"</span>,
    onClick = <span class="kw">function</span>()
        <span class="nb">log</span>(<span class="s">"Clicked!"</span>)
    <span class="kw">end</span>,
})</code></pre>

            <div class="highlight">
                <p><strong>💡 Recomendación:</strong> Usa nombres de función global (string) para onClick. Las funciones anónimas se guardan en <code>_AXO_CALLBACKS</code> con un ID generado por <code>to_pointer()</code>, pero pueden perderse entre hot reloads.</p>
            </div>

            <h2>onChangeText</h2>
            <p>Disponible en <strong>TextInput</strong>. Se dispara cuando el texto cambia:</p>
            <pre><code>UI.TextInput({
    text = <span class="s">""</span>,
    onChangeText = <span class="s">"handleTextChange"</span>,
})

<span class="kw">function</span> <span class="nf">handleTextChange</span>(newText)
    <span class="nb">log</span>(<span class="s">"Nuevo texto: "</span> .. newText)
<span class="kw">end</span></code></pre>

            <h2>log()</h2>
            <p>Función global disponible en Lua para imprimir en la consola de Rust:</p>
            <pre><code><span class="nb">log</span>(<span class="s">"mensaje"</span>)
<span class="nb">log</span>(<span class="s">"valor: "</span> .. variable)</code></pre>

            <h2>Flujo de eventos</h2>
            <ol>
                <li>Lua crea el árbol de UI con callbacks (onClick como string o function)</li>
                <li>Rust parsea el árbol y asigna IDs de callback</li>
                <li>El hit test en la ventana detecta clics y busca el callback por ID</li>
                <li>Rust llama a la función global Lua correspondiente</li>
            </ol>
        `,
    },
    "device-api": {
        title: "Device API",
        description: "Acceso a hardware y sistema desde Lua.",
        content: `
            <p>Axo expone una API completa de device a través del objeto global <code>Device</code> y wrappers en <code>UI</code>.</p>

            <h2>Funciones del Device</h2>
            <table>
                <tr><th>Función</th><th>Retorna</th><th>Descripción</th></tr>
                <tr><td><code>Device.info()</code></td><td>table</td><td>Información del sistema (OS, versión, arquitectura)</td></tr>
                <tr><td><code>Device.checkPermission(name)</code></td><td>string</td><td>Estado de un permiso ("granted"/"denied")</td></tr>
                <tr><td><code>Device.requestPermission(name)</code></td><td>string</td><td>Solicita un permiso</td></tr>
                <tr><td><code>Device.getLocation()</code></td><td>table/nil</td><td>Coordenadas GPS (lat, lon)</td></tr>
                <tr><td><code>Device.getSensors()</code></td><td>table</td><td>Datos de sensores (accelerometer, gyroscope, magnetometer)</td></tr>
                <tr><td><code>Device.readFile(path)</code></td><td>string/nil</td><td>Lee archivo del storage</td></tr>
                <tr><td><code>Device.writeFile(path, content)</code></td><td>bool</td><td>Escribe archivo</td></tr>
                <tr><td><code>Device.deleteFile(path)</code></td><td>bool</td><td>Elimina archivo</td></tr>
                <tr><td><code>Device.fileExists(path)</code></td><td>bool</td><td>Verifica si archivo existe</td></tr>
                <tr><td><code>Device.storagePath()</code></td><td>string</td><td>Ruta base de almacenamiento</td></tr>
                <tr><td><code>Device.showNotification(title, body)</code></td><td>bool</td><td>Muestra notificación</td></tr>
                <tr><td><code>Device.takePhoto()</code></td><td>string/nil</td><td>Toma foto (retorna ruta)</td></tr>
            </table>

            <h2>Wrappers en UI</h2>
            <p>La std library también provee wrappers:</p>
            <pre><code><span class="c">-- Obtener información del dispositivo</span>
<span class="kw">local</span> info = UI.getDeviceInfo()
<span class="nb">log</span>(info.os_name)

<span class="c">-- Verificar permisos</span>
<span class="kw">local</span> status = UI.checkPermission(<span class="s">"location"</span>)

<span class="c">-- Acceder al objeto Device directamente</span>
<span class="kw">local</span> dev = UI.Device()
dev.getLocation()</code></pre>

            <h2>Ejemplo</h2>
            <pre><code><span class="kw">function</span> <span class="nf">App</span>()
    <span class="kw">local</span> info = UI.getDeviceInfo()
    <span class="kw">return</span> UI.View({
        style = { ... },
        children = {
            UI.Text({
                text = <span class="s">"OS: "</span> .. (info.os_name <span class="kw">or</span> <span class="s">"unknown"</span>),
            }),
        },
    })
<span class="kw">end</span></code></pre>
        `,
    },
    "permissions": {
        title: "Permisos",
        description: "Sistema de permisos con default Denied.",
        content: `
            <p>Axo tiene un sistema de permisos inspirado en Android/iOS. Todos los permisos tienen default <strong>Denied</strong> por seguridad.</p>

            <h2>Permisos disponibles</h2>
            <table>
                <tr><th>Nombre</th><th>Descripción</th></tr>
                <tr><td><code>"location"</code></td><td>Acceso a geolocalización GPS</td></tr>
                <tr><td><code>"camera"</code></td><td>Acceso a cámara</td></tr>
                <tr><td><code>"storage"</code></td><td>Acceso a almacenamiento de archivos</td></tr>
                <tr><td><code>"sensors"</code></td><td>Acceso a sensores (acelerómetro, etc.)</td></tr>
                <tr><td><code>"notifications"</code></td><td>Enviar notificaciones</td></tr>
            </table>

            <h2>Uso</h2>
            <pre><code><span class="c">-- Verificar estado</span>
<span class="kw">local</span> status = Device.checkPermission(<span class="s">"location"</span>)
<span class="c">-- "denied", "granted", o "unavailable"</span>

<span class="c">-- Solicitar permiso</span>
<span class="kw">if</span> status == <span class="s">"denied"</span> <span class="kw">then</span>
    status = Device.requestPermission(<span class="s">"location"</span>)
<span class="kw">end</span>

<span class="c">-- Usar si fue concedido</span>
<span class="kw">if</span> status == <span class="s">"granted"</span> <span class="kw">then</span>
    <span class="kw">local</span> loc = Device.getLocation()
<span class="kw">end</span></code></pre>

            <h2>Arquitectura</h2>
            <p>Los permisos se implementan via el trait <code>PermissionHandler</code>, que permite intercambiar implementaciones:</p>
            <ul>
                <li><strong>DesktopPermissionHandler</strong> — default para Linux/macOS/Windows. Por ahora otorga todos los permisos si se solicitan.</li>
                <li><strong>MobilePermissionHandler</strong> — (futuro) para Android/iOS, con diálogos nativos.</li>
            </ul>
        `,
    },
    "hot-reload": {
        title: "Hot Reload",
        description: "Recarga tu UI sin reiniciar la app.",
        content: `
            <p>Axo incluye hot reload vía <strong>notify</strong> (file watcher). Cuando editas un archivo Lua en <code>app/</code>, la app se recarga automáticamente.</p>

            <h2>Cómo funciona</h2>
            <ol>
                <li>El CLI inicia un watcher en el directorio <code>app/</code></li>
                <li>Cuando detecta un cambio, recrea la VM de Lua completa</li>
                <li>Re-ejecuta <code>App()</code> y regenera el árbol de UI</li>
                <li>Taffy re-calcula el layout y wgpu re-renderiza</li>
            </ol>

            <div class="highlight">
                <p><strong>⚠️ Importante:</strong> Las variables de estado en Lua se pierden entre hot reloads. Para estado persistente, considera usar el sistema de callbacks o guardar en <code>Device</code> storage.</p>
            </div>

            <h2>Activación</h2>
            <p>El hot reload se activa automáticamente con <code>axo-cli dev</code>. No requiere configuración adicional.</p>

            <h2>Detalles técnicos</h2>
            <p>El watcher se implementa con <code>std::mem::forget(watcher)</code> para mantenerlo vivo sin loop infinito en el thread de recepción de eventos. Las funciones globales (como los onClick handlers) persisten entre recargas porque se re-registran al re-ejecutar <code>app.lua</code>.</p>

            <h2>Limitaciones</h2>
            <ul>
                <li>Las funciones anónimas en <code>_AXO_CALLBACKS</code> se pierden al recargar</li>
                <li>Estado en variables globales Lua se reinicia</li>
                <li>El engine Rust (ventana, wgpu) no se reinicia — solo la VM de Lua</li>
            </ul>
        `,
    },
    "style-props": {
        title: "Propiedades de estilo",
        description: "Referencia completa de propiedades de estilo.",
        content: `
            <p>Todas las propiedades de estilo disponibles en Axo.</p>

            <h2>Dimensiones</h2>
            <table>
                <tr><th>Propiedad</th><th>Tipo</th><th>Default</th><th>Descripción</th></tr>
                <tr><td><code>width</code></td><td>length | %</td><td>auto</td><td>Ancho</td></tr>
                <tr><td><code>height</code></td><td>length | %</td><td>auto</td><td>Alto</td></tr>
                <tr><td><code>minWidth</code></td><td>length | %</td><td>auto</td><td>Ancho mínimo</td></tr>
                <tr><td><code>maxWidth</code></td><td>length | %</td><td>auto</td><td>Ancho máximo</td></tr>
                <tr><td><code>minHeight</code></td><td>length | %</td><td>auto</td><td>Alto mínimo</td></tr>
                <tr><td><code>maxHeight</code></td><td>length | %</td><td>auto</td><td>Alto máximo</td></tr>
            </table>

            <h2>Flexbox</h2>
            <table>
                <tr><th>Propiedad</th><th>Valores</th><th>Default</th></tr>
                <tr><td><code>flexDirection</code></td><td><code>"row"</code>, <code>"column"</code>, <code>"rowReverse"</code>, <code>"columnReverse"</code></td><td><code>"column"</code></td></tr>
                <tr><td><code>justifyContent</code></td><td><code>"flexStart"</code>, <code>"flexEnd"</code>, <code>"center"</code>, <code>"spaceBetween"</code>, <code>"spaceAround"</code>, <code>"spaceEvenly"</code></td><td><code>"flexStart"</code></td></tr>
                <tr><td><code>alignItems</code></td><td><code>"flexStart"</code>, <code>"flexEnd"</code>, <code>"center"</code>, <code>"stretch"</code>, <code>"baseline"</code></td><td><code>"stretch"</code></td></tr>
                <tr><td><code>alignSelf</code></td><td><code>"auto"</code>, <code>"flexStart"</code>, <code>"flexEnd"</code>, <code>"center"</code>, <code>"stretch"</code>, <code>"baseline"</code></td><td><code>"auto"</code></td></tr>
                <tr><td><code>alignContent</code></td><td><code>"flexStart"</code>, <code>"flexEnd"</code>, <code>"center"</code>, <code>"stretch"</code>, <code>"spaceBetween"</code>, <code>"spaceAround"</code></td><td><code>"stretch"</code></td></tr>
                <tr><td><code>flexWrap</code></td><td><code>"nowrap"</code>, <code>"wrap"</code>, <code>"wrapReverse"</code></td><td><code>"nowrap"</code></td></tr>
                <tr><td><code>flexGrow</code></td><td>número</td><td><code>0</code></td></tr>
                <tr><td><code>flexShrink</code></td><td>número</td><td><code>1</code></td></tr>
                <tr><td><code>gap</code></td><td>número</td><td><code>0</code></td></tr>
            </table>

            <h2>Spacing</h2>
            <table>
                <tr><th>Propiedad</th><th>Formato</th><th>Descripción</th></tr>
                <tr><td><code>margin</code></td><td>número o "top right bottom left"</td><td>Margen externo. <code>"10"</code> = todos lados. <code>"10 5"</code> = vertical horizontal. <code>"10 5 10 5"</code> = individual.</td></tr>
                <tr><td><code>padding</code></td><td>número o "top right bottom left"</td><td>Padding interno. Mismo formato que margin.</td></tr>
            </table>

            <h2>Apariencia</h2>
            <table>
                <tr><th>Propiedad</th><th>Tipo</th><th>Descripción</th></tr>
                <tr><td><code>backgroundColor</code></td><td>hex</td><td>Color de fondo. Formato: <code>"#rgb"</code>, <code>"#rrggbb"</code>, <code>"#rrggbbaa"</code>.</td></tr>
                <tr><td><code>color</code></td><td>hex</td><td>Color de texto para componentes Text.</td></tr>
                <tr><td><code>fontSize</code></td><td>número</td><td>Tamaño de fuente en píxeles (default: 16).</td></tr>
                <tr><td><code>borderWidth</code></td><td>número</td><td>Ancho del borde en píxeles.</td></tr>
                <tr><td><code>borderColor</code></td><td>hex</td><td>Color del borde.</td></tr>
            </table>

            <h2>Posicionamiento</h2>
            <table>
                <tr><th>Propiedad</th><th>Valores</th><th>Descripción</th></tr>
                <tr><td><code>position</code></td><td><code>"relative"</code>, <code>"absolute"</code></td><td>Tipo de posicionamiento. <code>"absolute"</code> posiciona relativo al padre.</td></tr>
                <tr><td><code>top</code>, <code>left</code>, <code>right</code>, <code>bottom</code></td><td>length | %</td><td>Offset para posicionamiento absoluto.</td></tr>
            </table>
        `,
    },
    "architecture": {
        title: "Arquitectura",
        description: "Cómo está construido Axo.",
        content: `
            <p>Axo está organizado como un workspace de Cargo con 4 crates principales:</p>

            <h2>Crates</h2>
            <table>
                <tr><th>Crate</th><th>Propósito</th></tr>
                <tr><td><code>axo-core</code></td><td>Motor principal: renderer (wgpu), window (winit), layout (Taffy), text (ab_glyph), hot reload, permisos, device API</td></tr>
                <tr><td><code>axo-bridge</code></td><td>Capa de integración Lua-Rust: create_vm(), parseo de UI tree, conversión Taffy, device API wrapper</td></tr>
                <tr><td><code>axo-cli</code></td><td>CLI tooling: init, dev, build, release. Es el binary que instala el usuario.</td></tr>
                <tr><td><code>axo-platforms</code></td><td>Abstracciones platform-specific (futuro para mobile/web).</td></tr>
            </table>

            <h2>Flujo de ejecución</h2>
            <ol>
                <li><code>axo-cli dev</code> inicia el engine Rust</li>
                <li>Se crea una VM Lua y se ejecuta <code>app/app.lua</code></li>
                <li><code>App()</code> retorna un árbol de UI (tablas anidadas)</li>
                <li>El bridge parsea las tablas a <code>UiNode</code> (Rust structs)</li>
                <li>Taffy calcula el layout (posiciones y tamaños)</li>
                <li>wgpu renderiza los rectángulos y texto</li>
                <li>Los clics del usuario se traducen a callbacks Lua</li>
                <li>Hot reload detecta cambios en <code>app/</code> y repite desde paso 2</li>
            </ol>

            <h2>Diagrama</h2>
            <pre><code>┌─────────────┐     ┌──────────────┐     ┌──────────┐
│  Lua App    │────▶│  axo-bridge  │────▶│ axo-core │
│ (UI tree)   │     │  (parseo)    │     │ (taffy)  │
└─────────────┘     └──────────────┘     └────┬─────┘
                                              │
                                     ┌────────▼────────┐
                                     │   wgpu Render   │
                                     │ (Vulkan/Metal)  │
                                     └─────────────────┘</code></pre>

            <h2>Principios de diseño</h2>
            <ul>
                <li><strong>2-capas:</strong> Lua App ↔ Rust Core. No hay dependencia circular: core no depende de bridge, bridge depende de core.</li>
                <li><strong>Shared state Arc&lt;Mutex&lt;Vec&lt;Rect&gt;&gt;&gt;</strong> para hot reload: file watcher actualiza rects desde thread separado, renderer los consume en cada frame.</li>
                <li><strong>Permisos por trait:</strong> PermissionHandler trait para intercambiar implementaciones desktop vs mobile.</li>
            </ul>
        `,
    },
    "adrs": {
        title: "Architecture Decision Records",
        description: "Decisiones arquitectónicas documentadas.",
        content: `
            <p>Las decisiones arquitectónicas importantes se documentan como ADRs en <code>docs/adr/</code>.</p>

            <h2>ADRs disponibles</h2>
            <table>
                <tr><th>ADR</th><th>Título</th><th>Descripción</th></tr>
                <tr><td>ADR-001</td><td>Estrategia de Renderizado</td><td>Por qué wgpu en lugar de Skia-rs o WebGPU directo</td></tr>
                <tr><td>ADR-002</td><td>Bridge Data Model</td><td>Cómo se comunican Lua y Rust</td></tr>
                <tr><td>ADR-003</td><td>Accesibilidad</td><td>Estrategia de accesibilidad cross-platform</td></tr>
                <tr><td>ADR-004</td><td>Web Target Lua</td><td>Cómo correr Lua en navegadores (Lua 5.4 a WASM, no LuaJIT)</td></tr>
                <tr><td>ADR-005</td><td>Mobile Lifecycle</td><td>Manejo del ciclo de vida en Android/iOS</td></tr>
            </table>

            <h2>Decisiones clave</h2>
            <ul>
                <li><strong>wgpu</strong> sobre Skia-rs: binarios más pequeños, control total del pipeline gráfico</li>
                <li><strong>mlua con Lua 5.4</strong>: send-safe, WASM-compatible (a diferencia de LuaJIT)</li>
                <li><strong>onClick como string</strong>: serializable, funciona entre hot reloads. Funciones anónimas se guardan en <code>_AXO_CALLBACKS</code> con ID de <code>to_pointer()</code></li>
                <li><strong>Font LiberationSans embebido</strong> via <code>include_bytes!</code> — no requiere archivos externos</li>
                <li><strong>Permissions default Denied</strong> por seguridad (cambio de Granted a Denied en v0.1.3)</li>
            </ul>
        `,
    },
};

var DEFAULT_PAGE = "intro";
var searchTimeout = null;

function renderSidebar() {
    var nav = document.getElementById("sidebarNav");
    nav.innerHTML = "";
    DOCS_DATA.forEach(function(cat) {
        var catDiv = document.createElement("div");
        catDiv.className = "sidebar-category";
        var label = document.createElement("div");
        label.className = "sidebar-category-label";
        label.textContent = cat.label;
        catDiv.appendChild(label);
        cat.pages.forEach(function(page) {
            var a = document.createElement("a");
            a.href = "#" + page.id;
            a.className = "sidebar-link";
            a.textContent = page.title;
            a.dataset.page = page.id;
            catDiv.appendChild(a);
        });
        nav.appendChild(catDiv);
    });
}

function navigate(pageId) {
    var page = DOCS_PAGES[pageId];
    if (!page) { navigate(DEFAULT_PAGE); return; }

    var content = document.getElementById("docsContent");
    content.innerHTML = `
        <h1>${page.title}</h1>
        <p class="page-description">${page.description}</p>
        ${page.content}
    `;

    document.querySelectorAll(".sidebar-link").forEach(function(a) {
        a.classList.toggle("active", a.dataset.page === pageId);
    });

    if (window.location.hash !== "#" + pageId) {
        history.pushState(null, "", "#" + pageId);
    }
}

function onHashChange() {
    var pageId = window.location.hash.replace("#", "") || DEFAULT_PAGE;
    navigate(pageId);
}

function setupSearch() {
    var input = document.getElementById("docSearch");
    input.addEventListener("input", function() {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(function() {
            var q = input.value.toLowerCase().trim();
            document.querySelectorAll(".sidebar-link").forEach(function(a) {
                var title = a.textContent.toLowerCase();
                var matches = !q || title.indexOf(q) !== -1;
                a.style.display = matches ? "block" : "none";
            });
        }, 200);
    });
}

function setupMobileToggle() {
    var toggle = document.createElement("button");
    toggle.className = "sidebar-toggle";
    toggle.innerHTML = "☰";
    toggle.addEventListener("click", function() {
        document.getElementById("sidebar").classList.toggle("open");
    });
    document.body.appendChild(toggle);

    document.querySelectorAll(".sidebar-link").forEach(function(a) {
        a.addEventListener("click", function() {
            document.getElementById("sidebar").classList.remove("open");
        });
    });
}

window.addEventListener("hashchange", onHashChange);
window.addEventListener("popstate", onHashChange);

renderSidebar();
setupSearch();
setupMobileToggle();
onHashChange();
