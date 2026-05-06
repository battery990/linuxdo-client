<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let { selected = $bindable(), onchange } = $props();
  let categories = $state([]);

  onMount(async () => {
    try {
      const data = await invoke("get_categories");
      categories = data.category_list?.categories ?? [];
    } catch (e) {
      console.error("Failed to load categories:", e);
    }
  });
</script>

<div class="category-filter">
  <button
    class="chip"
    class:active={selected === null}
    onclick={() => {
      selected = null;
      onchange?.(null);
    }}
  >
    全部
  </button>
  {#each categories.slice(0, 10) as cat}
    <button
      class="chip"
      class:active={selected === cat.slug}
      onclick={() => {
        selected = cat.slug;
        onchange?.(cat.slug);
      }}
    >
      {cat.name}
    </button>
  {/each}
</div>

<style>
  .category-filter {
    display: flex;
    gap: 8px;
    padding: 10px 16px;
    overflow-x: auto;
    scrollbar-width: none;
    -webkit-overflow-scrolling: touch;
  }
  .category-filter::-webkit-scrollbar {
    display: none;
  }
  .chip {
    flex-shrink: 0;
    padding: 6px 14px;
    border-radius: 20px;
    font-size: 13px;
    font-weight: 500;
    background: var(--md-sys-color-surface-container);
    color: var(--md-sys-color-on-surface-variant);
    transition: all 0.15s;
  }
  .chip.active {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
  }
</style>
