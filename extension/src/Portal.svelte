<script lang="ts">
  import { tick } from "svelte";

  export let target: HTMLElement | string = "body";

  function portal(el: HTMLElement, target: HTMLElement | string = "body") {
    let targetEl;
    async function update(newTarget) {
      target = newTarget;
      if (typeof target === "string") {
        targetEl = document.querySelector(target);
        if (targetEl === null) {
          await tick();
          targetEl = document.querySelector(target);
        }
        if (targetEl === null) {
          throw new Error(
            `No element found matching css selector: "${target}"`
          );
        }
      } else if (target instanceof HTMLElement) {
        targetEl = target;
      } else {
        throw new TypeError(
          `Unknown portal target type: ${
            target === null ? "null" : typeof target
          }. Allowed types: string (CSS selector) or HTMLElement.`
        );
      }
      targetEl.appendChild(el);
      el.hidden = false;
    }
    function destroy() {
      if (el.parent) {
        el.parent.removeChild(el);
      }
    }
    update(target);
    return {
      update,
      destroy,
    };
  }
</script>

<div use:portal={target} hidden>
  <slot />
</div>
