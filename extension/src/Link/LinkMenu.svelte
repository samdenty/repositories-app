<script lang="ts">
  import Menu from "../Menu.svelte";
  import MenuItem from "../MenuItem.svelte";
  import type { LinkData, File, FolderLike } from "./parseLink";
  import MenuDivider from "../MenuDivider.svelte";
  import { request } from "../content/native";

  export let x: number;
  export let y: number;
  export let href: string;
  export let link: LinkData;

  function revealInFinder() {
    request({ request: "RevealInFinder", ...link });
  }

  function openFile() {
    request({ request: "OpenFile", ...(link as File) });
  }

  function openInTerminal() {
    request({ request: "OpenInTerminal", ...(link as FolderLike) });
  }

  function openInEditor() {
    request({ request: "OpenInEditor", ...link });
  }

  function copyPath() {}

  function getInfo() {}

  function quickLook() {}

  function openLinkInNewTab() {
    window.open(href);
  }

  function copyLinkAddress() {}
</script>

<Menu {x} {y} on:close>
  {#if link.type === "File"}
    <MenuItem on:click={openInEditor}
      >Open {(() => {
        const names = link.path.split("/");
        return `“${names[names.length - 1]}”`;
      })()}</MenuItem
    >
    <MenuItem>Open With</MenuItem>
    <MenuItem on:click={revealInFinder}>Reveal in Finder</MenuItem>
  {:else if link.type === "User"}
    <MenuItem on:click={revealInFinder}>Reveal in Finder</MenuItem>
    <MenuItem on:click={openInTerminal}>Open in Terminal</MenuItem>
  {:else if link.type === "Repo"}
    <MenuItem on:click={openInEditor}>Open repo in Editor</MenuItem>
    <MenuItem on:click={openInTerminal}>Open in Terminal</MenuItem>
    <MenuItem on:click={revealInFinder}>Reveal in Finder</MenuItem>
  {:else if link.type === "Folder"}
    <MenuItem on:click={revealInFinder}>Reveal in Finder</MenuItem>
    <MenuItem on:click={openInTerminal}>Open in Terminal</MenuItem>
  {/if}

  <MenuDivider />

  <MenuItem on:click={openLinkInNewTab}>Open Link in New Tab</MenuItem>
  <MenuItem on:click={copyLinkAddress}>Copy Link Address</MenuItem>

  <MenuDivider />

  <MenuItem on:click={copyPath}>Copy Path</MenuItem>
  <MenuItem on:click={getInfo}>Get Info</MenuItem>
  <MenuItem on:click={quickLook}>Quick Look</MenuItem>

  <MenuDivider />

  <MenuItem>Services</MenuItem>
</Menu>
