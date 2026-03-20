<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  interface GameState {
    units: any[];
    tiles: Record<string, string>;
    terrain_types: Record<string, any>;
    locations: Record<string, any>;
    cursor_q: number;
    cursor_r: number;
    selected_unit_idx: number;
    year: number;
    month: number;
    turn: string;
    log: string;
  }

  let state: GameState | null = null;
  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  let hexSize = 35;
  let offsetX = 100;
  let offsetY = 100;

  // ドラッグ操作用
  let isDragging = false;
  let lastX = 0;
  let lastY = 0;

  async function updateState() {
    state = await invoke<GameState>('get_game_state');
    render();
  }

  async function handleHexClick(q: number, r: number) {
    state = await invoke<GameState>('click_hex', { q, r });
    render();
  }

  async function handleTurnEnd() {
    state = await invoke<GameState>('handle_input', { key: 't' });
    render();
  }

  function render() {
    if (!ctx || !state) return;
    ctx.fillStyle = '#001133';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    for (const [coord, terrainId] of Object.entries(state.tiles)) {
      const [q, r] = coord.split(',').map(Number);
      const { x, y } = hexToPixel(q, r);
      const terrain = state.terrain_types[terrainId];
      drawHex(x, y, terrain?.color || '#333', coord === `${state.cursor_q},${state.cursor_r}`);
    }

    for (const [coord, loc] of Object.entries(state.locations)) {
      const [q, r] = coord.split(',').map(Number);
      const { x, y } = hexToPixel(q, r);
      ctx.fillStyle = "white";
      ctx.font = "10px sans-serif";
      ctx.textAlign = "center";
      ctx.fillText(loc.name, x, y + 5);
    }

    state.units.forEach((u, idx) => {
      const { x, y } = hexToPixel(u.q, u.r);
      const isSelected = state!.selected_unit_idx === idx;
      ctx.beginPath();
      ctx.arc(x, y, hexSize * 0.6, 0, Math.PI * 2);
      ctx.fillStyle = u.faction === 'South' ? '#e74c3c' : (u.faction === 'North' ? '#3498db' : '#f1c40f');
      ctx.fill();
      if (isSelected) {
        ctx.strokeStyle = 'white';
        ctx.lineWidth = 3;
        ctx.stroke();
      }
      ctx.fillStyle = 'white';
      ctx.font = 'bold 12px sans-serif';
      ctx.fillText(u.name.substring(0, 2), x, y + 4);
    });
  }

  function hexToPixel(q: number, r: number) {
    const x = hexSize * 1.5 * q + offsetX;
    const y = hexSize * Math.sqrt(3) * (r + (q % 2) / 2) + offsetY;
    return { x, y };
  }

  function pixelToHex(x: number, y: number) {
    const q = Math.round((x - offsetX) / (hexSize * 1.5));
    const r = Math.round((y - offsetY) / (hexSize * Math.sqrt(3)) - (q % 2) / 2);
    return { q, r };
  }

  function drawHex(x: number, y: number, color: string, isCursor: boolean) {
    ctx.beginPath();
    for (let i = 0; i < 6; i++) {
      const angle = (Math.PI / 3) * i;
      ctx.lineTo(x + hexSize * Math.cos(angle), y + hexSize * Math.sin(angle));
    }
    ctx.closePath();
    ctx.fillStyle = color;
    ctx.fill();
    ctx.strokeStyle = isCursor ? 'yellow' : '#555';
    ctx.lineWidth = isCursor ? 3 : 1;
    ctx.stroke();
  }

  onMount(() => {
    ctx = canvas.getContext('2d')!;
    const resize = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      render();
    };
    window.addEventListener('resize', resize);
    resize();
    updateState();

    const aiInterval = setInterval(async () => {
      if (state && state.turn !== 'South') {
        state = await invoke<GameState>('run_ai_turn');
        render();
      }
    }, 1500);

    return () => {
      window.removeEventListener('resize', resize);
      clearInterval(aiInterval);
    };
  });

  // ドラッグ移動の実装
  function startDrag(x: number, y: number) {
    isDragging = true;
    lastX = x;
    lastY = y;
  }
  function moveDrag(x: number, y: number) {
    if (!isDragging) return;
    offsetX += x - lastX;
    offsetY += y - lastY;
    lastX = x;
    lastY = y;
    render();
  }
  function endDrag() {
    isDragging = false;
  }

  // マウスイベント
  function handleMouseDown(e: MouseEvent) {
    startDrag(e.clientX, e.clientY);
  }
  function handleMouseMove(e: MouseEvent) {
    moveDrag(e.clientX, e.clientY);
  }
  function handleMouseUp(e: MouseEvent) {
    if (isDragging && Math.abs(e.clientX - lastX) < 5 && Math.abs(e.clientY - lastY) < 5) {
      const rect = canvas.getBoundingClientRect();
      const { q, r } = pixelToHex(e.clientX - rect.left, e.clientY - rect.top);
      handleHexClick(q, r);
    }
    endDrag();
  }

  // タッチイベント (スマホ用)
  function handleTouchStart(e: TouchEvent) {
    const t = e.touches[0];
    startDrag(t.clientX, t.clientY);
  }
  function handleTouchMove(e: TouchEvent) {
    const t = e.touches[0];
    moveDrag(t.clientX, t.clientY);
  }
  function handleTouchEnd(e: TouchEvent) {
    const t = e.changedTouches[0];
    const rect = canvas.getBoundingClientRect();
    const { q, r } = pixelToHex(t.clientX - rect.left, t.clientY - rect.top);
    // ドラッグ距離が小さければクリックとみなす
    handleHexClick(q, r);
    endDrag();
  }
</script>

<main>
  <canvas 
    bind:this={canvas} 
    on:mousedown={handleMouseDown}
    on:mousemove={handleMouseMove}
    on:mouseup={handleMouseUp}
    on:touchstart|passive={handleTouchStart}
    on:touchmove|passive={handleTouchMove}
    on:touchend|passive={handleTouchEnd}
  ></canvas>

  {#if state}
    <div class="ui-overlay">
      <div class="status-bar">
        <span>{state.year}年 {state.month}月</span>
        <span class={state.turn.toLowerCase()}>{state.turn}</span>
      </div>
      
      <div class="log-panel">
        <p>{state.log}</p>
      </div>

      <div class="controls">
        <button on:click={handleTurnEnd}>ターン終了</button>
      </div>
    </div>
  {/if}
</main>

<style>
  main {
    width: 100vw;
    height: 100vh;
    margin: 0;
    overflow: hidden;
    position: relative;
    background: #001133;
    touch-action: none;
  }
  canvas { display: block; }
  .ui-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: env(safe-area-inset-top) 10px env(safe-area-inset-bottom) 10px;
    box-sizing: border-box;
  }
  .status-bar {
    background: rgba(0,0,0,0.8);
    color: white;
    padding: 10px;
    display: flex;
    justify-content: space-between;
    border-radius: 8px;
    pointer-events: auto;
    font-size: 0.9em;
    border: 1px solid #444;
  }
  .log-panel {
    background: rgba(0,0,0,0.7);
    color: #eee;
    padding: 8px 12px;
    margin-top: auto;
    margin-bottom: 12px;
    border-radius: 8px;
    font-size: 0.85em;
    pointer-events: auto;
    border-left: 4px solid #e67e22;
  }
  .controls {
    display: flex;
    gap: 10px;
    pointer-events: auto;
  }
  button {
    flex: 1;
    background: #e67e22;
    color: white;
    border: none;
    padding: 15px;
    border-radius: 10px;
    font-weight: bold;
    font-size: 1em;
    box-shadow: 0 4px 0 #d35400;
  }
  button:active {
    transform: translateY(2px);
    box-shadow: 0 2px 0 #d35400;
  }
  .south { color: #ff5555; }
  .north { color: #55aaff; }
  .independent { color: #ffff55; }
</style>
