<script>
  import Portal from './Portal.svelte'
	import { onMount, setContext, createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { key } from './menu';

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
		dispatchClick: () => dispatch('click')
	});

	let menuEl;
  let isMounted = true;
	function onPageClick(e) {
		if (e.target === menuEl || menuEl.contains(e.target)) return;
		dispatch('clickoutside');
	}

  function onBlur() {
    console.log('blur')
    dispatch('clickoutside')
  }
</script>

<svelte:window on:blur={onBlur} on:pointerdown={onPageClick} on:contextmenu|preventDefault />
<svelte:head>
  <style>
    body {
      overflow: hidden;
      pointer-events: none;
    }
  </style>
</svelte:head>

<Portal>
  <div out:fade={{ duration: 100 }} bind:this={menuEl} style="top: {y}px; left: {x}px;">
    <slot />
  </div>
</Portal>

<style>
	div {
    pointer-events: initial;
    width: 288px;
    position: fixed;
    z-index: 999;
    display: grid;
    border: 1px solid #4f5154;
    padding: 4px;
    border-radius: 6px;
    box-shadow: 2px 2px 5px 0px #0002;
    background: #2f2f2f9e;
    backdrop-filter: blur(25px);
	}
</style>
