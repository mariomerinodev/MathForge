<script lang="ts">
  import { onMount } from 'svelte';
  import init, { solve } from '$lib/wasm/math_engine.js';
  import katex from 'katex';
  import { toLatex } from '$lib/mathTranslator';
  import 'katex/dist/katex.min.css'

  let expression = "2x + y = 10"; // Ejemplo nuevo para probar variables
  let result = ""; // Resultado crudo de Rust
  let renderedResult = ""; // Resultado HTML de KaTeX
  let isEngineReady = false;
  let isDarkMode = true; 
  let authState = 0;

  onMount(async () => {
    try {
      await init();
      isEngineReady = true;
      calculateResult(); // Calcular el ejemplo inicial
    } catch (e) {
      console.error("Error WASM:", e);
      result = "Error";
    }
    updateBodyTheme();
  });

  function handleButton(btn: string) {
    if (!isEngineReady) return;

    if (btn === 'C') {
      expression = "";
      result = "";
    } else if (btn === '⌫') {
      expression = expression.toString().slice(0, -1);
      if (expression === "") result = "";
    } else if (btn === '=') {
      calculateResult();
    } else {
      if (expression === "2x + y = 10" && result === "") {
          expression = "";
      }
      expression += btn;
    }
  }

  // Función reactiva: se ejecuta cada vez que cambia 'result'
  $: if (result) {
      // 1. Traducir de Rust a LaTeX
      const latex = toLatex(result);
      try {
          // 2. Renderizar LaTeX a HTML usando KaTeX
          renderedResult = katex.renderToString(latex, {
              displayMode: true, // Modo "display" para que se vea grande y centrado
              throwOnError: false // Si falla, muestra el texto crudo en vez de explotar
          });
      } catch (e) {
          renderedResult = result;
      }
  } else {
      renderedResult = "0";
  }


  function calculateResult() {
    if (expression.trim() === "") return;
    try {
      // Pequeña limpieza de entrada antes de enviar a Rust
      let parseableExpr = expression
        .replace(/×/g, '*')
        .replace(/÷/g, '/');
      result = solve(parseableExpr);
    } catch (e) {
      result = "Error";
    }
  }

  function toggleTheme() {
    isDarkMode = !isDarkMode;
    updateBodyTheme();
  }

  function updateBodyTheme() {
    if (typeof document !== 'undefined') {
        document.body.classList.toggle('light-mode', !isDarkMode);
    }
  }
</script>

<main>
  <nav class="navbar">
    <div class="logo">MathForge</div>
    <div class="nav-actions">
      <div class="auth-slider">
        <div class="slider-bg" style="transform: translateX({authState * 100}%);"></div>
        <button class="auth-btn {authState === 0 ? 'active' : ''}" on:click={() => authState = 0}>
            <span class="dot">{authState === 0 ? '●' : '○'}</span> Iniciar sesión
        </button>
        <button class="auth-btn {authState === 1 ? 'active' : ''}" on:click={() => authState = 1}>
            <span class="dot">{authState === 1 ? '●' : '○'}</span> Registrarse
        </button>
      </div>
      <button class="theme-btn" on:click={toggleTheme}>
        <div class="theme-icon-container {isDarkMode ? 'show-moon' : 'show-sun'}">
            <div class="emoji">☀️</div>
            <div class="emoji">🌙</div>
        </div>
      </button>
    </div>
  </nav>

  <div class="content">
    <header>
        <h1>Calculadora Avanzada</h1>
        <p>Potencia matemática a tu alcance</p>
    </header>

    <div class="calc-wrapper">
        <div class="calculator">
            <div class="display-screen">
                <input 
                    type="text" 
                    class="expression-input" 
                    bind:value={expression} 
                    on:keydown={(e) => e.key === 'Enter' && calculateResult()}
                    placeholder="Escribe aquí..."
                    autocomplete="off"
                    spellcheck="false"
                />
                <div class="result-scroll">
                    <span class="result katex-container">{@html renderedResult || "0"}</span>
                </div>
            </div>

            <div class="keypad">
                {#each ['C', '(', ')', '÷'] as btn}
                    <button class="btn top-op" on:click={() => handleButton(btn)}>{btn}</button>
                {/each}
                {#each ['7', '8', '9', '×'] as btn}
                    <button class="btn {btn === '×' ? 'op' : 'num'}" on:click={() => handleButton(btn)}>{btn}</button>
                {/each}
                {#each ['4', '5', '6', '-'] as btn}
                    <button class="btn {btn === '-' ? 'op' : 'num'}" on:click={() => handleButton(btn)}>{btn}</button>
                {/each}
                {#each ['1', '2', '3', '+'] as btn}
                    <button class="btn {btn === '+' ? 'op' : 'num'}" on:click={() => handleButton(btn)}>{btn}</button>
                {/each}
                {#each ['0', '.', '⌫', '='] as btn}
                    <button class="btn {btn === '=' ? 'equals' : 'num'}" on:click={() => handleButton(btn)}>{btn}</button>
                {/each}
            </div>

            <div class="scientific-pad variable-row">
                {#each ['x', 'y', 'z', '^', '√'] as btn}
                    <button class="btn sci var-btn" on:click={() => handleButton(btn)}>{btn}</button>
                {/each}
            </div>
        </div>
    </div>
  </div>
</main>

<style>
  /* --- VARIABLES CSS --- */
  :global(:root) {
    --bg: #0a0a0a;
    --calc-bg: #141414;
    --display-bg: #050505;
    --btn-num: #1e1e1e;
    --btn-op: #2a2a2a;
    --text-main: #ffffff;
    --text-dim: #a3a3a3;
    --accent: #ff8c00; 
    --glow: rgba(255, 140, 0, 0.15);
    --nav-bg: #1c1c1c;
    --nav-text-active: #ffffff;
  }

  :global(body.light-mode) {
    --bg: #f4f6f9;
    --calc-bg: #ffffff;
    --display-bg: #f8f9fa;
    --btn-num: #f0f3f8;
    --btn-op: #e2e8f0;
    --text-main: #1a1a1a;
    --text-dim: #64748b;
    --accent: #3b82f6; 
    --glow: rgba(59, 130, 246, 0.2);
    --nav-bg: #e2e8f0;
    --nav-text-active: #ffffff;
  }

  /* --- GLOBAL RESET --- */
  :global(html), :global(body) {
    height: 100vh;
    margin: 0;
    padding: 0;
    overflow: hidden; 
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  :global(body) {
    background-color: var(--bg);
    color: var(--text-main);
    transition: background-color 0.4s ease, color 0.4s ease;
  }

  main {
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 2rem 3rem;
    box-sizing: border-box;
  }

  /* --- NAVBAR & AUTH SLIDER (Sin cambios) --- */
  .navbar { display: flex; justify-content: space-between; align-items: center; flex-shrink: 0; }
  .logo { font-size: 1.4rem; font-weight: 800; color: var(--accent); transition: color 0.4s ease; }
  .nav-actions { display: flex; gap: 1.5rem; align-items: center; }
  .auth-slider { position: relative; display: flex; background-color: var(--nav-bg); border-radius: 2rem; padding: 4px; width: 330px; transition: background-color 0.4s ease; }
  .slider-bg { position: absolute; top: 4px; left: 4px; width: calc(50% - 4px); height: calc(100% - 8px); background-color: var(--accent); border-radius: 2rem; transition: transform 0.4s cubic-bezier(0.25, 1, 0.5, 1), background-color 0.4s ease; z-index: 0; }
  .auth-btn { position: relative; z-index: 1; background: transparent; border: none; padding: 0.6rem 0; font-size: 0.9rem; font-weight: 600; color: var(--text-dim); cursor: pointer; width: 50%; display: flex; align-items: center; justify-content: center; gap: 8px; white-space: nowrap; transition: color 0.3s ease; }
  .auth-btn.active { color: var(--nav-text-active); }
  .dot { font-size: 1rem; line-height: 1; }
  .theme-btn { background: transparent; border: 1px solid var(--accent); border-radius: 50%; width: 42px; height: 42px; cursor: pointer; overflow: hidden; position: relative; transition: border-color 0.4s ease; }
  .theme-icon-container { position: absolute; top: 0; left: 0; width: 100%; height: 200%; display: flex; flex-direction: column; align-items: center; transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1); }
  .show-moon { transform: translateY(-50%); }
  .show-sun { transform: translateY(0); }
  .emoji { width: 100%; height: 50%; display: flex; align-items: center; justify-content: center; font-size: 1.1rem; }

  /* --- CONTENIDO CENTRAL --- */
  .content { flex-grow: 1; display: flex; flex-direction: column; justify-content: center; align-items: center; }
  header { text-align: center; margin-bottom: 2rem; }
  header h1 { font-size: 2rem; margin: 0 0 0.5rem 0; }
  header p { color: var(--text-dim); margin: 0; transition: color 0.4s ease; }

  /* --- CALCULADORA --- */
  .calc-wrapper { box-shadow: 0 0 80px 10px var(--glow); border-radius: 1.5rem; transition: box-shadow 0.4s ease; }
  .calculator { background: var(--calc-bg); width: 320px; border-radius: 1.5rem; padding: 1.5rem; display: flex; flex-direction: column; gap: 1rem; transition: background-color 0.4s ease; }

  /* PANTALLA SCREEN */
  .display-screen {
    background-color: var(--display-bg);
    border-radius: 1rem;
    padding: 1.2rem;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 0.5rem;
    border: 1px solid rgba(128, 128, 128, 0.1);
    box-shadow: inset 0 4px 15px rgba(0, 0, 0, 0.2);
    margin-bottom: 0.5rem;
    transition: background-color 0.4s ease;
    min-height: 110px; /* Altura mínima para que quepa bien el resultado de KaTeX */
  }

  .expression-input { width: 100%; background: transparent; border: none; text-align: right; font-size: 1.1rem; color: var(--text-dim); font-family: inherit; outline: none; padding: 0; margin: 0; transition: color 0.4s ease; }
  .expression-input::placeholder { color: rgba(163, 163, 163, 0.3); }

  /* RESULTADO KATEX */
  .result-scroll { width: 100%; overflow-x: auto; overflow-y: hidden; text-align: right; white-space: nowrap; scrollbar-width: none; -ms-overflow-style: none; }
  .result-scroll::-webkit-scrollbar { display: none; }

  /* Ajustes específicos para el contenedor de KaTeX */
  .katex-container {
    font-size: 1.8rem; /* Tamaño base */
    color: var(--accent);
    display: inline-block; /* Necesario para que el text-align: right funcione bien */
  }
  /* Hacemos que los elementos internos de KaTeX hereden el color de acento */
  :global(.katex) { color: inherit !important; }


  /* BOTONES */
  .keypad { display: grid; grid-template-columns: repeat(4, 1fr); gap: 0.6rem; }
  .scientific-pad { display: grid; grid-template-columns: repeat(5, 1fr); gap: 0.6rem; margin-top: 0.2rem; }

  /* Estilos para los nuevos botones de variables */
  .var-btn {
      font-family: 'Times New Roman', serif; /* Las variables matemáticas suelen ser serif e itálicas */
      font-style: italic;
      font-weight: 600;
  }

  .btn { font-family: inherit; font-size: 1.2rem; border: none; border-radius: 0.6rem; cursor: pointer; height: 55px; transition: all 0.2s cubic-bezier(0.25, 0.8, 0.25, 1); }
  .btn:hover { transform: translateY(-2px); filter: brightness(1.15); }
  .btn:active { transform: translateY(1px) scale(0.96); filter: brightness(0.9); }
  .btn.num { background: var(--btn-num); color: var(--text-main); }
  .btn.op, .btn.top-op { background: var(--btn-op); color: var(--accent); }
  .btn.top-op { color: var(--text-dim); font-size: 1rem; }
  .btn.equals { background: var(--accent); color: white; font-size: 1.5rem; }
  .btn.sci { background: transparent; color: var(--text-dim); font-size: 1rem; height: 40px; } /* Aumentado un poco el tamaño de fuente para x,y,z */
  .btn.sci:hover { background: var(--btn-num); color: var(--text-main); }
</style>