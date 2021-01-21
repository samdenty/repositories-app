<script lang="ts">
  import LinkMenu from "./LinkMenu.svelte";
  import type { LinkData } from "./parseLink";
  import { request } from "../content/native";
  import { onMount } from "svelte";

  export let href: string;
  export let link: LinkData;
  export let childNodes: Node[];

  let children: HTMLElement;
  onMount(() => {
    children.append(...childNodes);
  });

  let pos = { x: 0, y: 0 };
  let showMenu = false;

  let isKeyDown = false;
  let isHovered = false;
  $: hovered = isKeyDown && isHovered;
</script>

{#if showMenu}
  <LinkMenu
    {...pos}
    {link}
    {href}
    on:close={() => {
      showMenu = false;
    }}
  />
{/if}

<svelte:body
  on:keyup={(e) => {
    if (e.key !== "Alt") return;
    isKeyDown = false;
  }}
  on:keydown={(e) => {
    if (e.key !== "Alt") return;
    isKeyDown = true;
  }} />

<span
  bind:this={children}
  on:contextmenu|preventDefault={async (e) => {
    if (showMenu) {
      showMenu = false;
      await new Promise((res) => setTimeout(res, 100));
    }

    pos = { x: e.clientX, y: e.clientY };
    showMenu = true;
  }}
  on:click={(e) => {
    if (!e.altKey) return;
    e.preventDefault();
    e.stopPropagation();

    request({ request: "OpenInEditor", ...link });
  }}
  on:mouseover={() => {
    isHovered = true;
  }}
  on:mouseout={() => {
    isHovered = false;
  }}
  class:hovered
/>

<style>
  span {
    display: contents;
  }

  .hovered {
    color: red;
  }
</style>
