<script lang="ts">
  import Menu from '../Menu.svelte';
  import MenuItem from '../MenuItem.svelte';
  import MenuDivider from '../MenuDivider.svelte';
  import { LinkData } from './parseLink'

  export let href: string;
  export let link: LinkData;
  export let name: string;
  export let children: string;

  let pos = { x: 0, y: 0 };

  let showMenu = false;

  async function onRightClick(e) {
    if (showMenu) {
			showMenu = false;
			await new Promise(res => setTimeout(res, 100));
		}

		pos = { x: e.clientX, y: e.clientY };
		showMenu = true;
  }

  let isKeyDown = false;
  function onKeyDown(e) {
    if (e.key !== "Alt") return
    isKeyDown = true;
  }
  function onKeyUp(e) {
    if (e.key !== "Alt") return
    isKeyDown = false;
  }

  let isHovered = false;
  function onMouseOver() {
    isHovered = true;
  }
  function onMouseOut() {
    isHovered = false;
  }

  $: hovered = isKeyDown && isHovered;

  $: {
    console.log(hovered)
  }

  function closeMenu() {
    showMenu = false;
  }

  function fileName() {
    const names = link.path.split('/')
    return `“${names[names.length - 1]}”`
  }

  function revealInFinder() {}

  function openFile() {}

  function openInTerminal() {}

  function openInEditor() {}

  function copyPath() {}

  function getInfo() {}

  function quickLook() {}

  function openLinkInNewTab() {
    window.open(href)
  }

  function copyLinkAddress() {}
</script>

<svelte:body on:keyup={onKeyUp} on:keydown={onKeyDown} />

<span on:contextmenu|preventDefault={onRightClick} on:mouseover={onMouseOver} on:mouseout={onMouseOut} class:hovered>
  {@html children}
</span>

{#if showMenu}
	<Menu {...pos} on:click={closeMenu} on:clickoutside={closeMenu}>
    {#if link.type === "File"}
      <MenuItem on:click={openFile}>Open {fileName()}</MenuItem>
      <MenuItem>Open With</MenuItem>
      <MenuItem on:click={revealInFinder}>Reveal in Finder</MenuItem>
    {:else if link.type === "User"}
      <MenuItem on:click={revealInFinder}>Reveal user in Finder</MenuItem>
      <MenuItem on:click={openInTerminal}>Open user in Terminal</MenuItem>
    {:else if link.type === "Repo"}
      <MenuItem on:click={openInEditor}>Open repo in Editor</MenuItem>
      <MenuItem on:click={openInTerminal}>Open repo in Terminal</MenuItem>
      <MenuItem>Reveal repo in Finder</MenuItem>
    {:else if link.type === "Folder"}
      <MenuItem on:click={revealInFinder}>Reveal folder in Finder</MenuItem>
      <MenuItem on:click={openInTerminal}>Open folder in Terminal</MenuItem>
    {/if}

    <MenuDivider />

		<MenuItem on:click={copyPath}>Copy Path</MenuItem>
    <MenuItem on:click={getInfo}>Get Info</MenuItem>
    <MenuItem on:click={quickLook}>Quick Look</MenuItem>

    <MenuDivider />

    <MenuItem on:click={openLinkInNewTab}>Open Link in New Tab</MenuItem>
    <MenuItem on:click={copyLinkAddress}>Copy Link Address</MenuItem>

    <MenuDivider />

		<MenuItem>Services</MenuItem>
	</Menu>
{/if}

<style>
  span {
    display: contents;
  }

  .hovered {
    color: red;
  }
</style>
