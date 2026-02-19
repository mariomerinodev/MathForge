<script lang="ts">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  import init, { solve, count_tokens, get_ast_visual } from 'math-engine';

  let astVisual: string = "";
  let expressionInput: string = "10 + 5 * 2^2";
  let tokenCount: number = 0;
  let finalResult: number = 0;
  let error: string | null = null;
  let isEngineReady = false;

  onMount(async () => {
    await init('/math_engine_bg.wasm');
    isEngineReady = true;
  });

  $: if (browser && isEngineReady) {
    tokenCount = count_tokens(expressionInput);
    const rawResult = solve(expressionInput);
    
    if (expressionInput.trim() === "") {
      finalResult = 0;
      error = null;
    } else if (isNaN(rawResult)) {
      error = "Syntax Error";
      finalResult = 0;
    } else {
      error = null;
      finalResult = rawResult;
    }

    astVisual = get_ast_visual(expressionInput);
  }
</script>

<svelte:head>
  <script src="https://cdn.tailwindcss.com"></script>
</svelte:head>

<div class="min-h-screen bg-[#0f0f0f] bg-[radial-gradient(circle_at_top_right,#2d1b1b,#0f0f0f)] flex items-center justify-center p-4 font-sans text-gray-200">
  
  <main class="w-full max-w-xl bg-white/5 backdrop-blur-xl border border-white/10 rounded-[2rem] p-8 md:p-12 shadow-2xl">
    
    <header class="flex justify-between items-center mb-10">
      <h1 class="text-3xl font-black bg-gradient-to-br from-[#ff5f6d] to-[#ffc371] bg-clip-text text-transparent tracking-tighter">
        MathForge
      </h1>
      <div class="flex items-center gap-2 bg-black/30 px-3 py-1.5 rounded-full border border-white/5 text-[10px] uppercase tracking-widest text-gray-500">
        <div class="w-1.5 h-1.5 rounded-full {isEngineReady ? 'bg-green-500 shadow-[0_0_8px_#4caf50]' : 'bg-yellow-500'}"></div>
        {isEngineReady ? 'Engine Active' : 'Loading...'}
      </div>
    </header>

    <section class="space-y-3">
      <div class="relative group">
        <input 
          type="text" 
          bind:value={expressionInput} 
          placeholder="Enter expression..."
          class="w-full bg-black/20 border border-white/10 rounded-2xl px-6 py-5 text-xl font-mono text-white outline-none focus:border-[#ff5f6d]/50 focus:bg-black/40 transition-all placeholder:text-gray-700"
          spellcheck="false"
        />
        {#if expressionInput}
          <button 
            on:click={() => expressionInput = ""}
            class="absolute right-5 top-1/2 -translate-y-1/2 text-gray-600 hover:text-white text-2xl transition-colors"
          >
            ×
          </button>
        {/if}
      </div>
      <div class="flex justify-between px-2 text-[11px] text-gray-600 font-medium uppercase tracking-wider">
        <span>Tokens detectados</span>
        <span>Count: <span class="text-gray-400 font-mono">{tokenCount}</span></span>
      </div>
    </section>

    <section class="mt-12 mb-8 min-h-[140px] flex flex-col items-center justify-center border-t border-white/5 pt-8">
      {#if error}
        <div class="flex items-center gap-2 text-[#ff5f6d] font-semibold animate-pulse">
          <span>⚠️</span> {error}
        </div>
      {:else}
        <span class="text-[10px] tracking-[0.3em] text-gray-600 font-bold mb-2">RESULTADO FINAL</span>
        <div class="text-6xl md:text-7xl font-mono font-bold bg-gradient-to-br from-[#ff5f6d] to-[#ffc371] bg-clip-text text-transparent">
          {finalResult}
        </div>
      {/if}
    </section>

    <footer class="mt-6 pt-6 border-t border-dashed border-white/5">
      <p class="text-[10px] text-gray-600 uppercase tracking-[0.2em] mb-4 text-center">Structure Analysis</p>
      
      <div class="bg-black/40 rounded-xl p-4 font-mono text-xs text-blue-300/70 overflow-x-auto whitespace-nowrap border border-white/5">
        {#if astVisual}
          <span class="text-gray-500">root</span> 
          <span class="text-white/20 mx-2">→</span> 
          {astVisual}
        {:else}
          <span class="text-gray-800 italic">Waiting for input...</span>
        {/if}
      </div>
    </footer>

  </main>
</div>

<style>
  /* Importamos una fuente más "pro" */
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;700;900&family=JetBrains+Mono:wght@500;700&display=swap');
  
  :global(body) {
    font-family: 'Inter', sans-serif;
  }
</style>