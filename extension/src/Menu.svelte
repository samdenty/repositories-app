<script>
  import Portal from "./Portal.svelte";
  import { setContext, createEventDispatcher } from "svelte";
  import { fade } from "svelte/transition";
  import { key } from "./menu";

  export let x;
  export let y;

  // whenever x and y is changed, restrict box to be within bounds
  $: (() => {
    if (!menuEl) return;

    const rect = menuEl.getBoundingClientRect();
    x = Math.min(window.innerWidth - rect.width, x);
    if (y > window.innerHeight - rect.height) y -= rect.height;
  })(x, y);

  const dispatch = createEventDispatcher();

  setContext(key, {
    dispatchClick: () => dispatch("close"),
  });

  let menuEl;
  let scale = getScale();

  function getScale() {
    const ratio =
      Math.round((window.outerWidth / window.innerWidth) * 100) / 100;

    return 1 / ratio;
  }

  let now = Date.now();
  function onPageClick(e) {
    if (e.target === menuEl || menuEl.contains(e.target)) return;
    dispatch("close");
  }
</script>

<svelte:window
  on:resize={() => {
    scale = getScale();
  }}
  on:blur={() => dispatch("close")}
  on:mousedown={onPageClick}
  on:keydown={(e) => {
    if (e.key !== "Escape") return;
    dispatch("close");
  }}
  on:mouseup={(e) => {
    if (Date.now() - now < 400) return;
    onPageClick(e);
  }}
  on:contextmenu|preventDefault
/>

<svelte:head
  ><style>
    body {
      overflow: hidden;
      pointer-events: none;
    }
  </style></svelte:head
>

<Portal>
  <div
    out:fade={{ duration: 100 }}
    bind:this={menuEl}
    style="top: {y}px; left: {x}px; transform: scale({scale})"
  >
    <slot />
  </div>
</Portal>

<style>
  div {
    pointer-events: initial;
    width: 288px;
    position: fixed;
    z-index: 2147483647;
    user-select: none;
    display: grid;
    border: 1px solid #4f5154;
    padding: 4px;
    transform-origin: top left;
    border-radius: 6px;
    box-shadow: 2px 2px 5px 0px #0002;
    background: #2f2f2f9e;
    backdrop-filter: blur(25px);
  }
</style>
