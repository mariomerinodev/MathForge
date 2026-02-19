<script lang="ts">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  // Importamos las funciones que ya tenemos en Rust
  import init, { solve, count_tokens } from 'math-engine';

  // Estados de la aplicación
  let expressionInput: string = "10 + 5 * 2";
  let tokenCount: number = 0;
  let finalResult: number = 0;
  let error: string | null = null;
  let isEngineReady = false;

  // Inicialización del motor WASM
  onMount(async () => {
    try {
      await init('/math_engine_bg.wasm');
      isEngineReady = true;
    } catch (e) {
      console.error("Failed to load WASM engine:", e);
      error = "Engine error: Could not load WebAssembly";
    }
  });

  // Bloque reactivo: Se ejecuta cada vez que cambia 'expressionInput'
  // siempre que el motor esté listo y estemos en el navegador.
  $: if (browser && isEngineReady) {
    // 1. Contamos tokens (incluyendo casos vacíos gracias al fix en Rust)
    tokenCount = count_tokens(expressionInput);
    
    // 2. Intentamos resolver
    const rawResult = solve(expressionInput);
    
    // 3. Validamos el resultado (f64::NAN desde Rust)
    if (expressionInput.trim() === "") {
      finalResult = 0;
      error = null;
    } else if (isNaN(rawResult)) {
      error = "Sintaxis inválida";
      finalResult = 0;
    } else {
      error = null;
      finalResult = rawResult;
    }
  }
</script>

<main class="container">
  <header>
    <h1>MathForge <span class="version">v0.2</span></h1>
    <p>Rust-powered Mathematical Engine</p>
  </header>

  <div class="calculator-card">
    <div class="input-group">
      <label for="math-input">Expresión Matemática</label>
      <input 
        id="math-input"
        type="text"
        bind:value={expressionInput} 
        placeholder="Ej: (10 + 5) / 2"
        spellcheck="false"
        autocomplete="off"
      />
    </div>

    <div class="info-bar">
      <span class="badge">Tokens: {tokenCount}</span>
      <span class="status {isEngineReady ? 'ready' : 'loading'}">
        {isEngineReady ? '● Motor Listo' : '○ Cargando...'}
      </span>
    </div>

    <div class="result-display">
      {#if error}
        <div class="error-msg">
          <span class="icon">⚠️</span> {error}
        </div>
      {:else}
        <div class="result-value">
          <small>Resultado:</small>
          <span class="number">{finalResult}</span>
        </div>
      {/if}
    </div>
  </div>

  <footer>
    <p>Tests pasados: <strong>6</strong> | Lenguaje: <strong>Rust + Svelte</strong></p>
  </footer>
</main>

<style>
  :global(body) {
    background-color: #f0f2f5;
    margin: 0;
    font-family: 'Inter', -apple-system, sans-serif;
  }

  .container {
    max-width: 500px;
    margin: 60px auto;
    padding: 20px;
  }

  header {
    text-align: center;
    margin-bottom: 30px;
  }

  h1 {
    margin: 0;
    color: #1a1a1a;
    font-weight: 800;
  }

  .version {
    font-size: 0.9rem;
    background: #ff3e00;
    color: white;
    padding: 2px 8px;
    border-radius: 12px;
    vertical-align: middle;
  }

  .calculator-card {
    background: white;
    padding: 30px;
    border-radius: 20px;
    box-shadow: 0 10px 25px rgba(0,0,0,0.05);
  }

  .input-group label {
    display: block;
    margin-bottom: 10px;
    color: #666;
    font-size: 0.9rem;
  }

  input {
    width: 100%;
    padding: 15px;
    font-size: 1.5rem;
    border: 2px solid #eee;
    border-radius: 12px;
    outline: none;
    transition: border-color 0.2s;
    box-sizing: border-box;
    font-family: 'JetBrains Mono', monospace;
  }

  input:focus {
    border-color: #ff3e00;
  }

  .info-bar {
    display: flex;
    justify-content: space-between;
    margin-top: 15px;
    font-size: 0.8rem;
  }

  .badge {
    background: #f0f0f0;
    padding: 4px 10px;
    border-radius: 6px;
    color: #444;
  }

  .status.ready { color: #4CAF50; }
  .status.loading { color: #ff9800; }

  .result-display {
    margin-top: 30px;
    min-height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f9f9f9;
    border-radius: 12px;
  }

  .result-value {
    text-align: center;
  }

  .result-value small {
    display: block;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 1px;
    font-size: 0.7rem;
  }

  .number {
    font-size: 2.5rem;
    font-weight: 700;
    color: #1a1a1a;
  }

  .error-msg {
    color: #d32f2f;
    font-weight: 600;
  }

  footer {
    margin-top: 40px;
    text-align: center;
    color: #aaa;
    font-size: 0.8rem;
  }
</style>