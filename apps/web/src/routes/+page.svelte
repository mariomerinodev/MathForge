<script lang="ts">
  import { onMount } from 'svelte';
  import init, { saludar_motor, sumar_test } from 'math-engine';

  let mensaje = 'Iniciando sistema...';
  let resultado = 0;
  let cargado = false;

  onMount(async () => {
    try {
      // Como el archivo está en 'static', en la web vive en '/'
      await init('/math_engine_bg.wasm'); 
      
      mensaje = saludar_motor("Mario");
      resultado = sumar_test(10, 5);
      cargado = true;
    } catch (e) {
      console.error("Error al cargar:", e);
      mensaje = "Error al conectar con Rust.";
    }
  });
</script>

<main>
  <div class="card">
    <h1>MathForge</h1>
    <p class:ready={cargado}>{mensaje}</p>
    {#if cargado}
       <p class="debug">Cálculo desde Rust: 10 + 5 = {resultado}</p>
    {/if}
  </div>
</main>

<style>
  :global(body) { background: #121212; color: white; font-family: sans-serif; }
  .card { border: 1px solid #333; padding: 2rem; border-radius: 12px; margin-top: 50px; text-align: center; }
  .ready { color: #4caf50; font-weight: bold; }
  .debug { color: #888; font-family: monospace; }
</style>