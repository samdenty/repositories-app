<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { onMount, getContext } from "svelte";
  import { key } from "./menu";

  export let isDisabled = false;
  export let text = "";

  const dispatch = createEventDispatcher();

  const { dispatchClick } = getContext(key);
</script>

<div
  class:disabled={isDisabled}
  on:mouseup={(e) => {
    if (isDisabled) return;

    dispatch("click");
    dispatchClick();
  }}
>
  {#if text}
    {text}
  {:else}
    <slot />
  {/if}
</div>

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
