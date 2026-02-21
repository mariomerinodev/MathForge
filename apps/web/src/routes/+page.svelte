<script lang="ts">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  
  import init, { solve, get_ast_visual, count_tokens } from '../../../../packages/math-engine/src/lib/wasm/math_engine.js';

  let expressionInput = "2x + 4x = 12";
  let finalResult: any = "0";
  let astVisual = "";
  let tokenCount = 0;
  let isEngineReady = false;
  let error: string | null = null;

  onMount(async () => {
    if (browser) {
      try {
        await init();
        isEngineReady = true;
        updateAnalysis();
      } catch (e) {
        console.error("Error al inicializar el motor WASM:", e);
        error = "Error de motor";
      }
    }
  });

  function updateAnalysis() {
    if (!isEngineReady) return;

    try {
      tokenCount = count_tokens(expressionInput);
      
      // Rust devuelve String, así que finalResult siempre será String
      const result = solve(expressionInput);

      if (expressionInput.trim() === "") {
        finalResult = "0";
        error = null;
      } else {
        finalResult = result as any;
        error = null;
      }

      astVisual = get_ast_visual(expressionInput);
    } catch (e) {
      error = "Error de sintaxis";
      finalResult = "0";
    }
  }

  // Reactividad: se ejecuta cuando cambia el input o el motor está listo
  $: if (browser && isEngineReady && expressionInput !== undefined) {
    updateAnalysis();
  }
</script>

<svelte:head>
  <script src="https://cdn.tailwindcss.com"></script>
</svelte:head>

<main class="min-h-screen bg-[#1a1614] text-white p-4 flex flex-col items-center justify-center font-sans">
  
  <div class="mb-6 flex items-center gap-2 bg-neutral-800/50 px-3 py-1 rounded-full border border-neutral-700">
    <div class="w-2 h-2 rounded-full {isEngineReady ? 'bg-green-500 animate-pulse' : 'bg-red-500'}"></div>
    <span class="text-[10px] uppercase tracking-tighter text-neutral-400 font-bold">
      {isEngineReady ? 'Engine Active' : 'Engine Offline'}
    </span>
  </div>

  <div class="w-full max-w-lg bg-[#25211f] rounded-3xl p-8 shadow-2xl border border-neutral-800">
    <h1 class="text-3xl font-black mb-8 text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-rose-500 italic">
      MathForge
    </h1>

    <div class="relative group">
      <input 
        type="text" 
        bind:value={expressionInput}
        placeholder="Escribe una ecuación..."
        class="w-full bg-[#1a1614] border-2 border-neutral-700 rounded-2xl p-5 text-xl font-medium focus:outline-none focus:border-orange-500 transition-all placeholder:text-neutral-600"
      />
      <button 
        on:click={() => expressionInput = ""}
        aria-label="Borrar contenido"
        title="Borrar"
        class="absolute right-4 top-1/2 -translate-y-1/2 text-neutral-500 hover:text-white transition-colors"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <div class="mt-2 flex justify-between px-2">
      <span class="text-[10px] text-neutral-500 uppercase font-bold tracking-widest">Tokens</span>
      <span class="text-[10px] text-neutral-500 font-mono italic">{tokenCount} detectados</span>
    </div>

    <div class="h-[1px] bg-neutral-800 my-8 w-full"></div>

    <div class="text-center py-4">
      <p class="text-[10px] text-neutral-500 uppercase font-bold tracking-[0.2em] mb-4">Resultado Final</p>
      {#if error}
        <p class="text-2xl font-bold text-rose-500">{error}</p>
      {:else}
        <p class="text-7xl font-black text-white tracking-tighter drop-shadow-lg">
          {finalResult}
        </p>
      {/if}
    </div>

    <div class="mt-8">
      <div class="bg-[#1a1614] rounded-xl p-4 border border-neutral-800/50">
        <p class="text-[9px] text-neutral-600 uppercase font-bold mb-3 tracking-widest">Structure Analysis</p>
        <div class="flex items-center gap-3">
          <div class="h-4 w-[2px] bg-orange-500/50 rounded-full"></div>
          <code class="text-orange-200/80 font-mono text-sm break-all">
            {astVisual || 'Esperando entrada...'}
          </code>
        </div>
      </div>
    </div>
  </div>

  <p class="mt-8 text-neutral-600 text-[10px] font-medium tracking-widest uppercase">
    Monorepo Bridge: Apps & Packages
  </p>
</main>

<style>
  :global(body) {
    background-color: #1a1614;
    margin: 0;
    overflow: hidden;
  }
</style>