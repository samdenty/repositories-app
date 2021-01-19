<script lang="ts">
	import { onMount, getContext } from 'svelte';
	import { key } from './menu';

	export let isDisabled = false;
	export let text = '';

	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	const { dispatchClick } = getContext(key);

	const handleClick = e => {
		if (isDisabled) return;

		dispatch('click');
		dispatchClick();
	}
</script>

<style lang="scss">
	div {
    color: #e9e9ea;
		padding: 2px 19px;
    font-family: -apple-system, system-ui, BlinkMacSystemFont;
		cursor: default;
		font-size: 13px;
    line-height: 18px;
		display: flex;
		align-items: center;
    border-radius: 5px;
		grid-gap: 5px;

    &:hover {
      background: #1453b3;
    }

    &.disabled {
      color: #0006;
      &:hover {
        background: white;
      }
    }
	}
</style>

<div
  class:disabled={isDisabled}
  on:click={handleClick}
>
	{#if text}
		{text}
	{:else}
		<slot />
	{/if}
</div>
