<script lang="ts">
  import { onMount } from 'svelte';
  // Importamos usando el alias $lib que apunta a apps/web/src/lib
  import init, { solve, get_ast_visual, count_tokens } from '$lib/wasm/math_engine.js';

  // Estados de la aplicación
  let expressionInput = "2x + 4x = 12";
  let finalResult = "0";
  let astVisual = "";
  let tokenCount = 0;
  let isEngineReady = false;
  let error: string | null = null;

  function updateAnalysis() {
    if (!isEngineReady) return;

    try {
      if (expressionInput.trim() === "") {
        finalResult = "0";
        astVisual = "";
        tokenCount = 0;
        error = null;
        return;
      }

      tokenCount = count_tokens(expressionInput);
      astVisual = get_ast_visual(expressionInput);
      finalResult = solve(expressionInput);
      
      error = null;
    } catch (e) {
      error = "Escribiendo...";
    }
  }

  onMount(async () => {
    try {
      await init();
      isEngineReady = true;
      console.log("🚀 Math Engine (WASM) listo");
      updateAnalysis();
    } catch (e) {
      console.error("❌ Error al cargar el motor:", e);
      error = "Error al cargar el motor WASM";
    }
  });
</script>

<main>
  <header>
    <h1>MATHFORGE</h1>
    <p>WASM Algebraic Engine</p>
  </header>

  <section class="input-container">
    <div class="input-card">
      <input 
        type="text"
        bind:value={expressionInput}
        on:input={updateAnalysis}
        placeholder="Ej: 2x + 5 = 15"
      />
      <div class="token-info">
        <span class="label">Tokens detectados</span>
        <span class="badge">{tokenCount} tokens</span>
      </div>
    </div>
  </section>

  <section class="result-section">
    <p class="section-label">Ecuación Procesada</p>
    {#if error}
      <p class="error-text">{error}</p>
    {:else}
      <p class="result-text">{finalResult}</p>
    {/if}
  </section>

  {#if astVisual}
    <section class="ast-section">
      <div class="ast-card">
        <h3>Structure Analysis (AST)</h3>
        <code>{astVisual}</code>
      </div>
    </section>
  {/if}
</main>

<style>
  /* CONFIGURACIÓN GLOBAL */
  :global(body) {
    background-color: #121212;
    margin: 0;
    color: white;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  main {
    max-width: 700px;
    margin: 0 auto;
    padding: 4rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 3.5rem;
  }

  /* CABECERA */
  header h1 {
    font-size: 3.5rem;
    font-weight: 900;
    color: #f97316; /* Naranja */
    font-style: italic;
    letter-spacing: -3px;
    margin: 0;
    text-align: center;
  }

  header p {
    color: #555;
    text-transform: uppercase;
    letter-spacing: 5px;
    font-size: 0.75rem;
    text-align: center;
    margin-top: 0.5rem;
  }

  /* TARJETA DE ENTRADA */
  .input-card {
    background: #1a1a1a;
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 2rem;
    border-radius: 1.5rem;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
  }

  input {
    width: 100%;
    background: transparent;
    border: none;
    color: white;
    font-size: 2.2rem;
    font-weight: 600;
    outline: none;
    padding: 0;
  }

  input::placeholder {
    color: #333;
  }

  .token-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 1.5rem;
    padding-top: 1.2rem;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
  }

  .label {
    color: #666;
    text-transform: uppercase;
    font-size: 0.65rem;
    font-weight: 800;
    letter-spacing: 1px;
  }

  .badge {
    background: rgba(249, 115, 22, 0.15);
    color: #f97316;
    padding: 0.3rem 0.8rem;
    border-radius: 2rem;
    font-size: 0.7rem;
    font-weight: 700;
  }

  /* SECCIÓN DE RESULTADO */
  .result-section {
    text-align: center;
  }

  .section-label {
    color: #555;
    text-transform: uppercase;
    font-size: 0.65rem;
    font-weight: 800;
    letter-spacing: 2px;
    margin-bottom: 1rem;
  }

  .result-text {
    font-size: 4.5rem;
    font-weight: 900;
    letter-spacing: -3px;
    margin: 0;
    color: white;
    line-height: 1;
  }

  .error-text {
    font-size: 2rem;
    font-weight: 700;
    color: #f97316;
    opacity: 0.6;
  }

  /* SECCIÓN AST */
  .ast-card {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.05);
    padding: 1.5rem;
    border-radius: 1.2rem;
  }

  .ast-card h3 {
    color: #444;
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 2px;
    margin-bottom: 1rem;
    margin-top: 0;
  }

  code {
    display: block;
    background: rgba(0, 0, 0, 0.4);
    padding: 1.2rem;
    border-radius: 0.8rem;
    color: #fdba74;
    font-family: 'Courier New', Courier, monospace;
    font-size: 0.9rem;
    overflow-x: auto;
    white-space: nowrap;
  }
</style>